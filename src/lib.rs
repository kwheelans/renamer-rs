#![warn(missing_docs)]
//! # Rename-rs
//! A library to process and rename files or text
//!
//! # Example
//! ```rust
//! # use renamer_rs::{Renamed, Delimiter, Selector, Format, Error};
//!
//! # fn run(delimiter: Delimiter, selector: Selector, format: Format, file: std::path::PathBuf) -> Result<Vec<Box<dyn Renamed>>, Error> {
//! let processor = renamer_rs::ProcessorBuilder::new(format)
//!          .delimiter(delimiter)
//!          .selector(selector)
//!         .file(file);
//!     processor.process()
//! }
//!     
//! ```

mod error;
mod processor;

pub use crate::error::Error;
pub use crate::processor::ProcessorBuilder;
pub use crate::processor::delimiter::{Delimiter, DelimiterType};
pub use crate::processor::extractor::Extractor;
pub use crate::processor::format::{Format, FormatPattern, FormatType};
pub use crate::processor::rename::{FileRenamer, RenameProcessor, Renamed};
pub use crate::processor::replacer::Replacer;
pub use crate::processor::selector::Selector;
pub use crate::processor::trim::Trim;
