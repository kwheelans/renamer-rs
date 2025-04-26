use crate::error::Error;
use regex::Regex;
use std::fmt::{Display, Formatter};

/// USed with [`Delimiter`] to indicate what type of processing should be used
#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub enum DelimiterType {
    /// Delimiter to be processes as a plain [`String`]
    String,
    /// Delimiter to be processes as a [`Regex`]
    Regex,
}

/// Represents a delimiter that will be used to process the input value into segments
#[derive(Debug, Clone)]
pub struct Delimiter {
    delimiter_type: DelimiterType,
    value: String,
    regex: Option<Regex>,
}

impl Delimiter {
    /// Create a  new [`Delimiter`]
    pub fn new<S: AsRef<str>>(value: S, delimiter_type: DelimiterType) -> Result<Self, Error> {
        match delimiter_type {
            DelimiterType::String => Ok(Self {
                delimiter_type,
                value: value.as_ref().into(),
                regex: None,
            }),
            DelimiterType::Regex => Ok(Self {
                delimiter_type,
                value: value.as_ref().into(),
                regex: Some(Regex::new(value.as_ref())?),
            }),
        }
    }

    /// Split a provided input value based on the [`Delimiter`] configuration
    pub fn split<S: AsRef<str>>(&self, input: S) -> Vec<String> {
        match self.delimiter_type {
            DelimiterType::String => input
                .as_ref()
                .split(self.value.as_str())
                .map(|s| s.to_string())
                .collect(),
            DelimiterType::Regex => self
                .regex
                .as_ref()
                .unwrap()
                .split(input.as_ref())
                .map(|s| s.to_string())
                .collect(),
        }
    }
}

impl Display for Delimiter {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value.as_str())
    }
}
