#[derive(Debug, Copy, Clone)]
enum TrimType {
    Left,
    Right,
    Both,
}

#[derive(Debug, Clone)]
pub struct Trim {
    direction: TrimType,
    pattern: String,
}

impl Trim {
    pub fn both<S: AsRef<str>>(pattern: S) -> Self {
        Self {
            direction: TrimType::Both,
            pattern: pattern.as_ref().into(),
        }
    }

    pub fn left<S: AsRef<str>>(pattern: S) -> Self {
        Self {
            direction: TrimType::Left,
            pattern: pattern.as_ref().into(),
        }
    }

    pub fn right<S: AsRef<str>>(pattern: S) -> Self {
        Self {
            direction: TrimType::Right,
            pattern: pattern.as_ref().into(),
        }
    }
    
    pub fn trim<S: AsRef<str>>(&self, value: S) -> String {
        match self.direction {
            TrimType::Left => {
                value.as_ref().trim_start_matches(&self.pattern).to_string()
            }
            TrimType::Right => {
                value.as_ref().trim_end_matches(&self.pattern).to_string()
            }
            TrimType::Both => {
                value.as_ref().trim_start_matches(&self.pattern).trim_end_matches(&self.pattern).to_string()
            }
        }
    }
    
    pub fn trim_slice<S: AsRef<str>>(&self, values: &[S]) -> Vec<String> {
        values.iter().map(|v| self.trim(v)).collect()
    }
}
