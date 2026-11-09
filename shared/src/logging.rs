/*
 *  SRP - Secure Reverse Proxy
 *  Copyright (C) 2026  SRP Team
 *
 *  This program is free software: you can redistribute it and/or modify
 *  it under the terms of the GNU Affero General Public License as
 *  published by the Free Software Foundation, either version 3 of the
 *  License, or (at your option) any later version.
 *
 *  This program is distributed in the hope that it will be useful,
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *  GNU Affero General Public License for more details.
 *
 *  You should have received a copy of the GNU Affero General Public License
 *  along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

use log::{info, LevelFilter};
use std::path::Path;
use std::time::SystemTime;

use crate::errors::LoggingError;

pub fn stdout_log_level_filter_from_arguments(
    quiet: bool,
    verbose: bool,
    debug: bool,
) -> Result<LevelFilter, LoggingError> {
    if (quiet && verbose) || (quiet && debug) || (verbose && debug) {
        return Err(LoggingError::ConflictingArguments);
    }

    Ok(if quiet {
        LevelFilter::Error
    } else if verbose {
        LevelFilter::Info
    } else if debug {
        LevelFilter::Debug
    } else {
        LevelFilter::Warn
    })
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

    if let Some(file_logging_path) = file_logging_path {
        let file = fern::log_file(file_logging_path)?;

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
    if let Some(file_logging_path) = file_logging_path {
        info!(
            "logging to file {:?} with level {}",
            file_logging_path,
            file_logging_level_filter.unwrap_or(LevelFilter::Warn)
        );
    }

    Ok(())
}
