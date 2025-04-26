use regex::Regex;

/// Represents a [`Regex`]  that is used to find a single matching segment
#[derive(Debug, Clone)]
pub struct Selector {
    #[allow(unused)]
    name: Option<String>, // todo: use name/id in format string
    pattern: Regex,
}

impl Selector {
    /// Create a new [`Selector`]
    pub fn new(name: Option<String>, pattern: Regex) -> Self {
        Self { name, pattern }
    }

    /// Returns true when a segment matches the provided pattern
    pub fn is_match<S: AsRef<str>>(&self, segment: S) -> bool {
        self.pattern.is_match(segment.as_ref())
    }

    /// Returns the first segment that matches the provided pattern
    pub fn match_segment<S: AsRef<str>>(&self, segments: &[S]) -> Option<String> {
        segments
            .iter()
            .position(|s| self.is_match(s))
            .map(|i| segments[i].as_ref().into())
    }
}
