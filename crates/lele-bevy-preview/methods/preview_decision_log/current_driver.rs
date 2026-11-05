#[must_use]
pub fn current_driver() -> String {
    let raw = std::env::var("VK_ICD_FILENAMES").unwrap_or_default();
    let file = std::path::Path::new(&raw)
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or_default();
    if file.is_empty() {
        String::from("default")
    } else {
        String::from(file)
    }
}

#[cfg(test)]
mod tests {
    use super::current_driver;

    #[test]
    fn test_usage() {
        let _ = current_driver();
    }
}
