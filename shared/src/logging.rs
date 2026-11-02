use log::{info, LevelFilter};
use std::path::Path;
use std::time::SystemTime;

pub fn stdout_log_level_filter_from_arguments(
    quiet: bool,
    verbose: bool,
    debug: bool,
) -> LevelFilter {
    if quiet {
        LevelFilter::Error
    } else if verbose {
        LevelFilter::Info
    } else if debug {
        LevelFilter::Debug
    } else {
        LevelFilter::Warn
    }
}

pub fn init(
    stdout_logging_level_filter: LevelFilter,
    file_logging_path: Option<&Path>,
    file_logging_level_filter: Option<LevelFilter>,
) -> Result<(), fern::InitError> {
    let mut logger = fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{} {} {} ({})",
                humantime::format_rfc3339_seconds(SystemTime::now()),
                record.level(),
                message,
                record.target(),
            ))
        })
        .level(LevelFilter::Trace)
        .chain(
            fern::Dispatch::new()
                .level(stdout_logging_level_filter)
                .chain(std::io::stdout()),
        );

    if file_logging_path.is_some() {
        let file = fern::log_file(file_logging_path.unwrap())?;

        logger = logger.chain(
            fern::Dispatch::new()
                .level(file_logging_level_filter.unwrap_or(LevelFilter::Warn))
                .chain(file),
        );
    };

    logger.apply()?;

    info!(
        "logging to stdout with level {}",
        stdout_logging_level_filter
    );
    if file_logging_path.is_some() {
        info!(
            "logging to file {:?} with level {}",
            file_logging_path.unwrap(),
            file_logging_level_filter.unwrap_or(LevelFilter::Warn)
        );
    }

    Ok(())
}
