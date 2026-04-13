pub fn get_current_dir() -> String {
    let current_dir = std::env::current_dir();
    match current_dir {
        Ok(path_buff) => path_buff.display().to_string(),
        Err(e) => e.to_string(),
    }
}

pub fn get_username() -> String {
    std::env::var("USER").unwrap_or_else(|_| "unknown".to_string())
}
