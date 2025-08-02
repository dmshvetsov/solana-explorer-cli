use std::fmt;

use serde::{Serialize, Serializer};

pub struct PrettyString {
    value: String,
}

impl fmt::Display for PrettyString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl fmt::Debug for PrettyString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.value, f)
    }
}

impl From<String> for PrettyString {
    fn from(s: String) -> Self {
        Self {
            value: s.trim_end_matches('\0').into(),
        }
    }
}

impl Serialize for PrettyString {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.value.as_str())
    }
}
