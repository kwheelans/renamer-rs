use crate::Error;
use crate::{Format, FormatType};
use std::fmt::Debug;
use std::path::{Path, PathBuf};

const EMPTY_STR: &str = "";

/// This trait represents objects that can produce [`Renamed`] trait objects
pub trait RenameProcessor {
    /// Produce a [`Renamed`] object from a [`RenameProcessor`]
    fn rename(&self) -> Box<dyn Renamed>;
}

/// This trait represents that has had the renaming process completed
pub trait Renamed: Debug {
    /// Return the original value
    fn original(&self) -> &str;

    /// Return the new value
    fn future(&self) -> &str;

    /// Take an action to commit the renaming process to the underlying item(i.e. actually rename the file no the filesystem)
    fn action(&self) -> Result<(), Error>;
}

/// Represents a file for the purpose on implementing the [`RenameProcessor`] trait
#[derive(Debug)]
pub struct FileRenamer {
    segments: Vec<String>,
    selected: Vec<Option<String>>,
    extracted: Vec<Option<String>>,
    format: Format,
    original_path: PathBuf,
}

/// Represents a file for the purpose on implementing the [`Renamed`] trait
#[derive(Debug)]
pub struct RenamedFile {
    original_path: PathBuf,
    new_path: PathBuf,
    original_name: String,
    new_name: String,
}

#[derive(Debug)]
pub struct TextRenamer {
    segments: Vec<String>,
    selected: Vec<Option<String>>,
    extracted: Vec<Option<String>>,
    format: Format,
    original_string: String,
}

#[derive(Debug)]
pub struct RenamedText {
    original: String,
    new: String,
}

impl RenameProcessor for FileRenamer {
    fn rename(&self) -> Box<dyn Renamed> {
        let new_name = process_format(
            self.segments.as_slice(),
            self.selected.as_slice(),
            self.extracted.as_slice(),
            &self.format,
        );
        let renamed = RenamedFile::new(self.original_path.as_path(), new_name);
        Box::new(renamed)
    }
}

impl FileRenamer {
    /// Create a new [`FileRenamer`]
    pub fn new<P: AsRef<Path>>(
        original_path: P,
        segments: Vec<String>,
        selected: Vec<Option<String>>,
        extracted: Vec<Option<String>>,
        format: Format,
    ) -> Self {
        Self {
            segments,
            selected,
            extracted,
            format,
            original_path: original_path.as_ref().into(),
        }
    }
}

impl Renamed for RenamedFile {
    fn original(&self) -> &str {
        self.original_name()
    }

    fn future(&self) -> &str {
        self.new_name()
    }

    fn action(&self) -> Result<(), Error> {
        Ok(std::fs::rename(self.original_path(), self.new_path())?)
    }
}

impl RenamedFile {
    /// Create a new [`RenamedFile`]
    pub fn new<P: AsRef<Path>, S: AsRef<str>>(original_path: P, new_name: S) -> Self {
        let mut new_path = original_path.as_ref().to_path_buf();
        new_path.set_file_name(new_name.as_ref());
        Self {
            original_path: original_path.as_ref().to_path_buf(),
            original_name: filename_as_string_lossy(original_path.as_ref()),
            new_path,
            new_name: new_name.as_ref().to_string(),
        }
    }

    /// Return the original file path
    pub fn original_path(&self) -> &Path {
        self.original_path.as_path()
    }

    /// Return the original filename
    pub fn original_name(&self) -> &str {
        &self.original_name
    }

    /// Return the new file path
    pub fn new_path(&self) -> &Path {
        self.new_path.as_path()
    }

    /// Return the new filename
    pub fn new_name(&self) -> &str {
        &self.new_name
    }
}

impl TextRenamer {
    /// Create a new [`TextRenamer`]
    pub fn new<S: AsRef<str>>(
        original_string: S,
        segments: Vec<String>,
        selected: Vec<Option<String>>,
        extracted: Vec<Option<String>>,
        format: Format,
    ) -> Self {
        Self {
            segments,
            selected,
            extracted,
            format,
            original_string: original_string.as_ref().into(),
        }
    }
}

impl RenameProcessor for TextRenamer {
    fn rename(&self) -> Box<dyn Renamed> {
        let new_name = process_format(
            self.segments.as_slice(),
            self.selected.as_slice(),
            self.extracted.as_slice(),
            &self.format,
        );
        let renamed = RenamedText::new(self.original_string.as_str(), new_name.as_str());
        Box::new(renamed)
    }
}

impl RenamedText {
    /// Create a new [`RenamedText`]
    pub fn new<S: AsRef<str>>(original: S, new: S) -> Self {
        Self {
            original: original.as_ref().into(),
            new: new.as_ref().into(),
        }
    }
}

impl Renamed for RenamedText {
    fn original(&self) -> &str {
        self.original.as_str()
    }

    fn future(&self) -> &str {
        self.new.as_str()
    }

    fn action(&self) -> Result<(), Error> {
        Ok(())
    }
}

/// Get a filename from a provided path as a [`String`]
pub(crate) fn filename_as_string_lossy<P: AsRef<Path>>(path: P) -> String {
    path.as_ref()
        .file_name()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string()
}

// Create future from original
fn process_format(
    segments: &[String],
    selected: &[Option<String>],
    extracted: &[Option<String>],
    format: &Format,
) -> String {
    let mut output = format.value().to_string();

    for replacer in format.patterns() {
        match replacer.format_type() {
            FormatType::Delimiter => {
                let replacement_value = match segments.get(replacer.id()) {
                    None => EMPTY_STR.to_string(),
                    Some(value) => value.to_string(),
                };

                output = output.replace(replacer.pattern(), replacement_value.as_str());
            }
            FormatType::Extractor => {
                let replacement_value = match extracted.get(replacer.id()) {
                    Some(Some(value)) => value.to_string(),
                    _ => EMPTY_STR.to_string(),
                };
                output = output.replace(replacer.pattern(), replacement_value.as_str());
            }
            FormatType::Selector => {
                let replacement_value = match selected.get(replacer.id()) {
                    Some(Some(value)) => value.to_string(),
                    _ => EMPTY_STR.to_string(),
                };
                output = output.replace(replacer.pattern(), replacement_value.as_str());
            }
        }
    }
    output
}
