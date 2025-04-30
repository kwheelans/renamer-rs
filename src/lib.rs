#![warn(missing_docs)]
//! # Rename-rs
//! A library to process and rename files or text
//!
//! # Example
//! ```rust
//! # use renamer_rs::{Renamed, Delimiter, Selector, Format, Error, InputType};
//!
//! # fn run(delimiter: Delimiter, selector: Selector, format: Format, input: InputType) -> Result<Vec<Box<dyn Renamed>>, Error> {
//! let processor = renamer_rs::ProcessorBuilder::new(format)
//!         .delimiter(delimiter)
//!         .selector(selector)
//!         .input(input);
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
pub use crate::processor::inputs::{FileInput, InputType, TextInput};
pub use crate::processor::rename::{FileRenamer, RenameProcessor, Renamed};
pub use crate::processor::replacer::Replacer;
pub use crate::processor::selector::Selector;
pub use crate::processor::trim::Trim;
