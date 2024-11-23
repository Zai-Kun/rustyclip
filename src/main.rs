mod args_parser;
mod data_manager;
mod logger;

use args_parser::Commands;
use data_manager::DataManager;
use expanduser::expanduser;
use log::error;
use std::{
    error::Error,
    io::{self, Read},
    path::PathBuf,
};

const CACHE_PATH: &str = "~/.cache/rustyclip";

fn main() {
    if let Err(err) = run_app() {
        error!("Application error: {}", err);
    }
}

/// Main application entry point.
fn run_app() -> Result<(), Box<dyn Error>> {
    // Parse command-line arguments
    let cmd = args_parser::parse_args();

    // Expand and prepare cache folder
    let cache_folder = expand_path(CACHE_PATH)?;
    ensure_folder_exists(&cache_folder)?;
    let log_file = cache_folder.join("log");

    // Initialize logger
    logger::init_logger(&log_file)?;

    // Initialize the data manager
    let mut data_manager = DataManager::new()?;

    // Execute the appropriate command
    match cmd.command {
        Commands::List(_) => list_items(&data_manager),
        Commands::Store(_) => store_item(&mut data_manager),
        Commands::Clear(_) => clear_database(&mut data_manager),
        Commands::Remove(remove_cmd) => remove_item(&mut data_manager, remove_cmd.query),
        Commands::Get(get_cmd) => get_item(&data_manager, get_cmd.query),
    }
}

/// List all items in the database.
fn list_items(data_manager: &DataManager) -> Result<(), Box<dyn Error>> {
    for (index, item) in data_manager.manifest_data.iter().enumerate() {
        println!("{index}: {}", item.preview.replace("\n", "").trim());
    }
    Ok(())
}

/// Store a new item from standard input.
fn store_item(data_manager: &mut DataManager) -> Result<(), Box<dyn Error>> {
    let input_data = read_stdin_as_bytes()?;
    data_manager.add_item(&input_data)?;
    Ok(())
}

/// Clear all items in the database.
fn clear_database(data_manager: &mut DataManager) -> Result<(), Box<dyn Error>> {
    data_manager.clear_db()?;
    Ok(())
}

/// Remove an item based on a query.
fn remove_item(
    data_manager: &mut DataManager,
    query: Option<String>,
) -> Result<(), Box<dyn Error>> {
    let query_string = query.unwrap_or_else(|| {
        match String::from_utf8(read_stdin_as_bytes().unwrap_or_default()) {
            Ok(value) => value,
            Err(_) => String::new(), // Default to an empty string on error
        }
    });
    if query_string.is_empty() {
        return Ok(()); // No query, no action
    }

    let parsed_index = parse_query(&query_string)?;
    data_manager.remove_item(parsed_index)?;
    Ok(())
}

/// Retrieve an item based on a query.
fn get_item(data_manager: &DataManager, query: Option<String>) -> Result<(), Box<dyn Error>> {
    let query_string = query.unwrap_or_else(|| {
        match String::from_utf8(read_stdin_as_bytes().unwrap_or_default()) {
            Ok(value) => value,
            Err(_) => String::new(), // Default to an empty string on error
        }
    });

    if query_string.is_empty() {
        return Ok(()); // No query, no action
    }

    let parsed_index = parse_query(&query_string)?;
    if parsed_index >= data_manager.manifest_data.len() {
        return Err("Invalid position".into());
    }

    let item = &data_manager.manifest_data[parsed_index];
    println!(
        "{}\n{}",
        data_manager.data_folder.join(&item.file_name).display(),
        item.mime_type
    );

    Ok(())
}

/// Parse the query string and extract an index.
fn parse_query(query: &str) -> Result<usize, Box<dyn Error>> {
    let index_str = query.split(':').next().unwrap_or(query).trim();
    Ok(index_str.parse::<usize>()?)
}

/// Read data from standard input as bytes.
fn read_stdin_as_bytes() -> Result<Vec<u8>, io::Error> {
    let mut buffer = Vec::new();
    io::stdin().lock().read_to_end(&mut buffer)?;
    Ok(buffer)
}

/// Ensure the specified folder exists, creating it if necessary.
fn ensure_folder_exists(path: &PathBuf) -> Result<(), io::Error> {
    std::fs::create_dir_all(path)?;
    Ok(())
}

/// Expand a user-relative path to its absolute form.
fn expand_path(path: &str) -> Result<PathBuf, Box<dyn Error>> {
    Ok(expanduser(path)?.into())
}
