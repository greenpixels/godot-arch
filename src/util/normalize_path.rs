pub fn normalize_path(path: &str) -> String {
    let mut normalized_path = path.replace('\\', "/").trim_start_matches('/').to_owned();
    if !normalized_path.starts_with("./") {
        normalized_path = format!("./{}", normalized_path)
    }
    normalized_path
}
