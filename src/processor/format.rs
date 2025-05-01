use crate::Error;
use crate::Error::{NoFormattingPatterns, UnknownFormatType};
use log::{debug, trace};
use regex::Regex;

const FORMAT_PATTERN: &str = r"%[dse]\d+%";
const SELECTOR_TYPE_PREFIX: &str = "%s";
const DELIMITER_TYPE_PREFIX: &str = "%d";
const EXTRACTOR_TYPE_PREFIX: &str = "%e";

/// Represents the type for format string being referenced
#[derive(Debug, Copy, Clone)]
pub(super) enum FormatType {
    /// Represents a [`Delimiter`][crate::Delimiter]
    Delimiter,
    /// Represents a [`Extractor`][crate::Extractor]
    Extractor,
    /// Represents a [`Selector`][crate::Selector]
    Selector,
}

/// Represents detected format patterns that will be replaced during processing
#[derive(Debug, Clone)]
pub(super) struct FormatPattern {
    pattern: String,
    format_type: FormatType,
    id: usize,
}

/// Represents the provided format string and all the detected format patterns
#[derive(Debug, Clone)]
pub struct Format {
    value: String,
    patterns: Vec<FormatPattern>,
}

impl FormatPattern {
    /// Create a new [`FormatPattern`]
    pub(super) fn new<S: AsRef<str>>(pattern: S, format_type: FormatType, id: usize) -> Self {
        Self {
            pattern: pattern.as_ref().into(),
            format_type,
            id,
        }
    }

    /// Returns the [`FormatType`]
    pub(super) fn format_type(&self) -> FormatType {
        self.format_type
    }

    /// Returns the actual format pattern [`String`]
    pub fn pattern(&self) -> &str {
        &self.pattern
    }

    /// Returns the detected ID value used to reference the pattern in combination with the [`FormatType`]
    pub fn id(&self) -> usize {
        self.id
    }
}

impl Format {
    /// Create a new [`Format`] from a provided format string
    pub fn new<S: AsRef<str>>(value: S) -> Result<Self, Error> {
        Ok(Self {
            value: value.as_ref().into(),
            patterns: Self::get_format_patterns(value)?,
        })
    }

    fn get_format_patterns<S: AsRef<str>>(value: S) -> Result<Vec<FormatPattern>, Error> {
        let format_pattern = Regex::new(FORMAT_PATTERN)?;
        let format_matches: Vec<_> = format_pattern
            .find_iter(value.as_ref())
            .map(|m| m.as_str().to_string())
            .collect();
        trace!("{:?}", format_matches);
        if format_matches.is_empty() {
            return Err(NoFormattingPatterns);
        }

        let mut format_patterns = Vec::with_capacity(format_matches.len());
        for format_match in &format_matches {
            let format_type = get_format_type(format_match)?;
            let id = format_match
                .get(2..format_match.len() - 1)
                .unwrap_or_default()
                .parse::<usize>()?;
            format_patterns.push(FormatPattern::new(format_match, format_type, id - 1))
        }
        debug!("{:?}", format_patterns);
        Ok(format_patterns)
    }

    /// Return the format string
    pub fn value(&self) -> &str {
        &self.value
    }

    /// Return the detected format patterns
    pub(super) fn patterns(&self) -> &[FormatPattern] {
        self.patterns.as_slice()
    }
}

fn get_format_type<S: AsRef<str>>(value: S) -> Result<FormatType, Error> {
    match value.as_ref() {
        v if v.starts_with(DELIMITER_TYPE_PREFIX) => Ok(FormatType::Delimiter),
        v if v.starts_with(SELECTOR_TYPE_PREFIX) => Ok(FormatType::Selector),
        v if v.starts_with(EXTRACTOR_TYPE_PREFIX) => Ok(FormatType::Extractor),
        _ => Err(UnknownFormatType(value.as_ref().into())),
    }
}
