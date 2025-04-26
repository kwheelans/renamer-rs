use std::num::ParseIntError;
use thiserror::Error;

/// Errors returned by pass-it-on-command-line-client
#[derive(Error, Debug)]
pub enum Error {
    /// No valid formatting patterns where detected in the format string
    #[error("No formatting patterns were found in the format string")]
    NoFormattingPatterns,

    /// Invalid  value was found during processing
    #[error("Invalid Value: {0}")]
    InvalidValue(String),

    // ### Converting from other error types ###
    /// Pass-thru [`std::io::Error`].
    #[error("std::io Error: {0}")]
    StdIo(#[from] std::io::Error),

    /// Pass-thru [`regex::Error`].
    #[error("regex Error: {0}")]
    RegEx(#[from] regex::Error),

    /// Pass-thru [`ParseIntError`].
    #[error("ParseIntError Error: {0}")]
    ParseInt(#[from] ParseIntError),
}
