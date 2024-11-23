use core::str;
use expanduser::expanduser;
use image_meta;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use xxhash_rust::const_xxh3::xxh3_64;

use crate::{ensure_folder_exists, CACHE_PATH};

const PREVIEW_LENGTH: usize = 100;

#[derive(Debug, Serialize, Deserialize)]
pub struct ClipboardItem {
    pub file_name: String,
    pub preview: String,
    pub mime_type: String,
}

#[derive(Debug)]
pub struct DataManager {
    pub data_folder: PathBuf,
    pub manifest_file: PathBuf,
    pub manifest_data: Vec<ClipboardItem>,
}

impl DataManager {
    pub fn new() -> Result<Self, io::Error> {
        let expanded_cache_folder = expanduser(CACHE_PATH)?;
        let clipboard_manifest_file = expanded_cache_folder.join("clipboard_manifest.json");
        let clipboard_data_folder = expanded_cache_folder.join("clipboard_data");

        ensure_folder_exists(&clipboard_data_folder)?;
        let manifest_data = load_manifest(&clipboard_manifest_file).unwrap_or_default();

        Ok(Self {
            data_folder: clipboard_data_folder,
            manifest_file: clipboard_manifest_file,
            manifest_data,
        })
    }

    pub fn add_item(&mut self, data: &[u8]) -> Result<(), Box<dyn Error>> {
        let key = xxh3_64(data).to_string();

        if self.manifest_data.iter().any(|item| item.file_name == key) {
            return Ok(());
        }

        File::create(self.data_folder.join(&key))?.write_all(data)?;

        let (preview, mime_type) = generate_preview_and_mime_type(data)?;
        self.manifest_data.insert(
            0,
            ClipboardItem {
                file_name: key,
                preview,
                mime_type,
            },
        );
        self.write_manifest()?;
        Ok(())
    }

    pub fn remove_item(&mut self, position: usize) -> Result<(), io::Error> {
        if position >= self.manifest_data.len() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Invalid position",
            ));
        }

        let item = self.manifest_data.remove(position);
        fs::remove_file(self.data_folder.join(item.file_name))?;
        self.write_manifest()
    }

    pub fn write_manifest(&self) -> Result<(), io::Error> {
        let data = serde_json::to_string(&self.manifest_data)?;
        File::create(&self.manifest_file)?.write_all(data.as_bytes())
    }

    pub fn clear_db(&mut self) -> Result<(), io::Error> {
        for item in &self.manifest_data {
            fs::remove_file(self.data_folder.join(&item.file_name))?;
        }
        self.manifest_data.clear();
        self.write_manifest()
    }
}

fn load_manifest(data_path: &Path) -> Result<Vec<ClipboardItem>, io::Error> {
    if data_path.exists() {
        let file = File::open(data_path)?;
        Ok(serde_json::from_reader(file)?)
    } else {
        Ok(Vec::new())
    }
}

fn generate_preview_and_mime_type(data: &[u8]) -> Result<(String, String), Box<dyn Error>> {
    const BIN_DATA_FORMAT: &str = "[[binary data {size} {image_fmt} {dimensions}]]";

    if let Ok(preview) = str::from_utf8(data) {
        let preview = preview.trim();
        return Ok((
            preview[..(PREVIEW_LENGTH+1).min(preview.len())].to_string(),
            "text/plain".to_string(),
        ));
    }

    if let Some(kind) = infer::get(data) {
        let mime_type = kind.mime_type();
        if mime_type.starts_with("image/") {
            let meta = image_meta::load_from_buf(data)?;
            let preview = BIN_DATA_FORMAT
                .replace("{size}", &human_readable_size(data.len()))
                .replace("{image_fmt}", mime_type.split('/').last().unwrap())
                .replace(
                    "{dimensions}",
                    &format!("{}x{}", meta.dimensions.width, meta.dimensions.height),
                );

            return Ok((preview, mime_type.to_string()));
        }

        let preview = BIN_DATA_FORMAT
            .replace("{size}", &human_readable_size(data.len()))
            .replace(" {image_fmt}", "")
            .replace(" {dimensions}", "");

        return Ok((preview, mime_type.to_string()));
    }

    Ok((
        format!("[[UNKNOWN {}]]", human_readable_size(data.len())),
        "application/octet-stream".to_string(),
    ))
}

fn human_readable_size(size: usize) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB", "PB", "EB"];
    let mut size = size as f64;
    let mut unit = 0;

    while size >= 1024.0 && unit < UNITS.len() - 1 {
        size /= 1024.0;
        unit += 1;
    }

    format!("{:.2} {}", size, UNITS[unit])
}
