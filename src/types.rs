use std::fmt;

pub enum InfoValue {
    Available(String),
    Unavailable(String),
}

impl InfoValue {
    pub fn from_result(result: Result<String, std::io::Error>, reason: &str) -> InfoValue {
        match result {
            Ok(v) => InfoValue::Available(v),
            Err(_) => InfoValue::Unavailable(reason.to_string()),
        }
    }
}

impl fmt::Display for InfoValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            InfoValue::Available(v) => write!(f, "{}", v),
            InfoValue::Unavailable(reason) => write!(f, "{}", reason),
        }
    }
}
