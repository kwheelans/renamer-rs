#[derive(Debug, Copy, Clone)]
enum TrimType {
    Left,
    Right,
    Both,
}

/// Represents a [`String`] to be trimmed from a value based on the provided configuration
#[derive(Debug, Clone)]
pub struct Trim {
    direction: TrimType,
    pattern: String,
}

impl Trim {
    /// Create a [`Trim`] item that will remove values from both ends of a value
    pub fn both<S: AsRef<str>>(pattern: S) -> Self {
        Self {
            direction: TrimType::Both,
            pattern: pattern.as_ref().into(),
        }
    }

    /// Create a [`Trim`] item that will remove values from only the left side of a value
    pub fn left<S: AsRef<str>>(pattern: S) -> Self {
        Self {
            direction: TrimType::Left,
            pattern: pattern.as_ref().into(),
        }
    }

    /// Create a [`Trim`] item that will remove values from only the right side of a value
    pub fn right<S: AsRef<str>>(pattern: S) -> Self {
        Self {
            direction: TrimType::Right,
            pattern: pattern.as_ref().into(),
        }
    }

    /// Process the provided value based on the [`Trim`] configuration
    pub fn trim<S: AsRef<str>>(&self, value: S) -> String {
        match self.direction {
            TrimType::Left => value.as_ref().trim_start_matches(&self.pattern).to_string(),
            TrimType::Right => value.as_ref().trim_end_matches(&self.pattern).to_string(),
            TrimType::Both => value
                .as_ref()
                .trim_start_matches(&self.pattern)
                .trim_end_matches(&self.pattern)
                .to_string(),
        }
    }

    /// Process the provided value based on the [`Trim`] configuration for each value in the slice
    pub fn trim_slice<S: AsRef<str>>(&self, values: &[S]) -> Vec<String> {
        values.iter().map(|v| self.trim(v)).collect()
    }
}
