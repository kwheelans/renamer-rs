use crate::error::Error;
use crate::processor::format::{Format, FormatType};
use std::path::{Path, PathBuf};

const EMPTY_STR: &str = "";
pub trait Renamer {
    fn rename(&self) -> Box<dyn Renamed>;
}

pub trait Renamed {
    fn original(&self) -> &str;
    fn future(&self) -> &str;
    fn action(&self) -> Result<(), Error>;
}

#[derive(Debug)]
pub struct FileRenamer {
    segments: Vec<String>,
    selected: Vec<Option<String>>,
    extracted: Vec<Option<String>>,
    format: Format,
    original_path: PathBuf,
}

#[derive(Debug)]
pub struct RenamedFile {
    original_path: PathBuf,
    new_path: PathBuf,
    original_name: String,
    new_name: String,
}

impl Renamer for FileRenamer {
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
    pub fn new<P: AsRef<Path>, S: AsRef<str>>(original_path: P, new_name: S) -> Self {
        let mut new_path = original_path.as_ref().to_path_buf();
        new_path.set_file_name(new_name.as_ref());
        Self {
            original_path: original_path.as_ref().to_path_buf(),
            original_name: filename_as_string(original_path.as_ref()),
            new_path,
            new_name: new_name.as_ref().to_string(),
        }
    }

    pub fn original_path(&self) -> &Path {
        self.original_path.as_path()
    }

    pub fn original_name(&self) -> &str {
        &self.original_name
    }

    pub fn new_path(&self) -> &Path {
        self.new_path.as_path()
    }

    pub fn new_name(&self) -> &str {
        &self.new_name
    }
}

pub fn filename_as_string<P: AsRef<Path>>(path: P) -> String {
    path.as_ref()
        .file_name()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string()
}

// Create future from original
fn process_format(segments: &[String], selected: &[Option<String>], extracted: &[Option<String>],format: &Format) -> String {
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
