#[must_use]
pub fn current_time() -> String {
    let now = std::time::SystemTime::now();
    let since = now
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(0, |elapsed| elapsed.as_secs());
    since.to_string()
}

#[cfg(test)]
mod tests {
    use super::current_time;

    #[test]
    fn test_usage() {
        let _ = current_time();
    }
}
