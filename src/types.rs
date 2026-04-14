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
