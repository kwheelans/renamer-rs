use regex::Regex;

#[derive(Debug, Clone)]
pub struct Replacer {
    pattern: Regex,
    substitute: String,
}

impl Replacer {
    pub fn new<S: AsRef<str>>(pattern: Regex, substitute: S) -> Self {
        Self {
            pattern,
            substitute: substitute.as_ref().into(),
        }
    }
    
    pub fn replace<S: AsRef<str>>(&self, value: S) -> String {
        self.pattern.replace_all(value.as_ref(), self.substitute.as_str()).into()
    }

    pub fn replace_slice<S: AsRef<str>>(&self, values: &[S]) -> Vec<String> {
        values.iter().map(|v| self.replace(v)).collect()
    }
}
