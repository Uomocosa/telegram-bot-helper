use crate::TestLog;

pub fn line(test_log: &TestLog, message: &str) {
    let stamped = format!("{:?} {message}", std::time::SystemTime::now());
    test_log.append_if_open(&stamped);
    eprintln!("{stamped}");
}

#[cfg(test)]
mod tests {
    use super::line;
    use crate::TestLog;

    #[test]
    fn test_usage() {
        let log = TestLog::from("test_log_line_smoke");
        line(&log, "line smoke");
        let body = std::fs::read_to_string(&log.path).unwrap_or_default();
        assert!(body.contains("line smoke"));
        let _ = std::fs::remove_file(&log.path);
    }
}
