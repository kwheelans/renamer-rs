use crate::Error::InvalidValue;
use regex::Regex;

/// A struct to be used with the [`ProcessorBuilder`][crate::ProcessorBuilder] to select values from the original string value before segmentation
#[derive(Debug, Clone)]
pub struct Extractor {
    #[allow(unused)]
    name: Option<String>, // todo: use name/id in format string
    pattern: Regex,
}

impl Extractor {
    /// Create a new [`Extractor`]
    pub fn new(name: Option<String>, pattern: Regex) -> Self {
        Self { name, pattern }
    }

    /// Perform the matching on the provide value
    pub fn extract<S: AsRef<str>>(&self, value: S) -> Option<String> {
        self.pattern
            .find(value.as_ref())
            .map(|m| m.as_str().to_string())
    }
}

impl TryFrom<&[String]> for Extractor {
    type Error = crate::error::Error;
    fn try_from(value: &[String]) -> Result<Self, Self::Error> {
        let pattern = match value.first() {
            None => Err(InvalidValue(
                "Extractor requires at least RegEx pattern".to_string(),
            )),
            Some(p) => Ok(Regex::new(p.as_str())?),
        }?;
        let name = value.get(1).cloned();

        Ok(Self { name, pattern })
    }
}
