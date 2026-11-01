use log::LevelFilter;
use simple_logging::log_to_file;
use std::path::Path;

pub fn init(log_file_path: &Path, log_level_filter: LevelFilter) -> Result<(), std::io::Error> {
    log_to_file(log_file_path, log_level_filter)
}
