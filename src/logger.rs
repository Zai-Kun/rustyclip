use std::error::Error;
use std::path::PathBuf;

use log::LevelFilter;
use log4rs;
use log4rs::append::{console::ConsoleAppender, file::FileAppender};
use log4rs::config::{Appender, Config, Root};
use log4rs::encode::pattern::PatternEncoder;

const PATTERN: &str = "{d(%Y-%m-%d %H:%M:%S)} [{l}] - {m}{n}";

pub fn init_logger(log_file: &PathBuf) -> Result<(), Box<dyn Error>> {
    let console = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new(PATTERN)))
        .build();

    let log_file = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new(PATTERN)))
        .build(log_file)?;

    let config = Config::builder()
        .appender(Appender::builder().build("console", Box::new(console)))
        .appender(Appender::builder().build("file", Box::new(log_file)))
        .build(
            Root::builder()
                .appender("console")
                .appender("file")
                .build(LevelFilter::Info),
        )?;

    log4rs::init_config(config)?;
    Ok(())
}
