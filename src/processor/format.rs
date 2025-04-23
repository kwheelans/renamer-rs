use crate::error::Error;
use crate::error::Error::NoFormattingPatterns;
use log::{debug, trace};
use regex::Regex;

const FORMAT_PATTERN: &str = r"%[dse]\d+%";
const SELECTOR_TYPE_PREFIX: &str = "%s";
const DELIMITER_TYPE_PREFIX: &str = "%d";

#[derive(Debug, Copy, Clone)]
pub enum FormatType {
    Delimiter,
    Extractor,
    Selector,
}

#[derive(Debug, Clone)]
pub struct FormatPattern {
    pattern: String,
    format_type: FormatType,
    id: usize,
}

#[derive(Debug, Clone)]
pub struct Format {
    value: String,
    patterns: Vec<FormatPattern>,
}

impl FormatPattern {
    pub fn new<S: AsRef<str>>(pattern: S, format_type: FormatType, id: usize) -> Self {
        Self {
            pattern: pattern.as_ref().into(),
            format_type,
            id,
        }
    }
    pub fn format_type(&self) -> FormatType {
        self.format_type
    }

    pub fn pattern(&self) -> &str {
        &self.pattern
    }

    pub fn id(&self) -> usize {
        self.id
    }
}

impl Format {
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
            let format_type = get_format_type(format_match);
            let id = format_match
                .get(2..format_match.len() - 1)
                .unwrap_or_default()
                .parse::<usize>()?;
            format_patterns.push(FormatPattern::new(format_match, format_type, id - 1))
        }
        debug!("{:?}", format_patterns);
        Ok(format_patterns)
    }

    pub fn value(&self) -> &str {
        &self.value
    }

    pub fn patterns(&self) -> &[FormatPattern] {
        self.patterns.as_slice()
    }
}

fn get_format_type<S: AsRef<str>>(value: S) -> FormatType {
    match value.as_ref() {
        v if v.starts_with(DELIMITER_TYPE_PREFIX)  => FormatType::Delimiter,
        v if v.starts_with(SELECTOR_TYPE_PREFIX)  => FormatType::Delimiter,
        _ => FormatType::Extractor,
    }
}

