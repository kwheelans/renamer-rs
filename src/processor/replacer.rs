use regex::Regex;

/// Represents a [`Regex`]  that is used to replace matches with a provided substitution
#[derive(Debug, Clone)]
pub struct Replacer {
    pattern: Regex,
    substitute: String,
}

impl Replacer {
    /// Create a new [`Replacer`]
    pub fn new<S: AsRef<str>>(pattern: Regex, substitute: S) -> Self {
        Self {
            pattern,
            substitute: substitute.as_ref().into(),
        }
    }

    /// Replaces all matches with the provided substitute value
    pub fn replace<S: AsRef<str>>(&self, value: S) -> String {
        self.pattern
            .replace_all(value.as_ref(), self.substitute.as_str())
            .into()
    }

    /// Replaces all matches with the provided substitute value for each value in the slice
    pub fn replace_slice<S: AsRef<str>>(&self, values: &[S]) -> Vec<String> {
        values.iter().map(|v| self.replace(v)).collect()
    }
}
