use std::num::ParseIntError;
use thiserror::Error;

/// Errors returned by pass-it-on-command-line-client
#[derive(Error, Debug)]
pub enum Error {
    #[error("Directory is not accessible: {0}")]
    DirectoryAccess(String),

    #[error("No files found to process in the input directory")]
    NoFileFound,

    #[error("No formatting patterns were found in the format string")]
    NoFormattingPatterns,

    #[error("Invalid Value: {0}")]
    InvalidValue(String),

    // ### Converting from other error types ###
    /// Pass-thru [`std::io::Error`].
    #[error("std::io Error: {0}")]
    StdIo(#[from] std::io::Error),

    #[error("regex Error: {0}")]
    RegEx(#[from] regex::Error),

    #[error("ParseIntError Error: {0}")]
    ParseInt(#[from] ParseIntError),
}
