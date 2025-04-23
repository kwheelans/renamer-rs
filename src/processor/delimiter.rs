use crate::error::Error;
use regex::Regex;
use std::fmt::{Display, Formatter};

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub enum DelimiterType {
    String,
    Regex,
}

#[derive(Debug, Clone)]
pub struct Delimiter {
    delimiter_type: DelimiterType,
    value: String,
    regex: Option<Regex>,
}

impl Delimiter {
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
