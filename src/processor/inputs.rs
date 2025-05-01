use std::fmt::Debug;
use std::hash::Hash;
use std::path::{Path, PathBuf};

/// Enum used to pass inputs to the [`ProcessorBuilder`][crate::ProcessorBuilder]
#[derive(Debug, Eq, PartialEq, Hash)]
pub enum InputType {
    /// Represents file path input and contains [`FileInput`]
    File(FileInput),
    /// Represents plain text input and contains [`TextInput`]
    Text(TextInput),
}

/// Represents file path input in [`InputType`]
#[derive(Debug, Eq, PartialEq, Hash)]
pub struct FileInput {
    value: PathBuf,
}

/// Represents plain text input in [`InputType`]
#[derive(Debug, Eq, PartialEq, Hash)]
pub struct TextInput {
    value: String,
}

impl InputType {
    /// Create a new [`InputType`] of variant [`File`]
    pub fn new_file<P: AsRef<Path>>(value: P) -> Self {
        Self::File(FileInput::new(value))
    }

    /// Create a new [`InputType`] of variant [`Text`]
    pub fn new_text<S: AsRef<str>>(value: S) -> Self {
        Self::Text(TextInput::new(value))
    }
}

impl FileInput {
    /// Create a new [`FileInput`]
    pub fn new<P: AsRef<Path>>(value: P) -> Self {
        Self {
            value: value.as_ref().into(),
        }
    }

    /// Return value as a [`Path`]
    pub fn value(&self) -> &Path {
        &self.value
    }
}

impl TextInput {
    /// Create a new [`TextInput`]
    pub fn new<S: AsRef<str>>(value: S) -> Self {
        Self {
            value: value.as_ref().into(),
        }
    }

    /// Return value as a [`str`]
    pub fn value(&self) -> &str {
        &self.value
    }
}
