use log::LevelFilter;
use simple_logging::{log_to_file, log_to_stderr};
use std::path::Path;

pub fn init_std_logger(quiet: bool, verbose: bool, debug: bool) {
    if quiet {
        log_to_stderr(LevelFilter::Error)
    } else if debug {
        log_to_stderr(LevelFilter::Debug)
    } else if verbose {
        log_to_stderr(LevelFilter::Info)
    } else {
        log_to_stderr(LevelFilter::Warn)
    }
}

pub fn init_log_file_logger(
    log_file_path: &Path,
    log_level_filter: LevelFilter,
) -> Result<(), std::io::Error> {
    log_to_file(log_file_path, log_level_filter)
}
