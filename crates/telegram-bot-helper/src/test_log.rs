use std::fs::File;
use std::path::PathBuf;
use std::sync::Mutex;

use atomic_delegate_macros::atomic_delegates;

pub struct TestLog {
    pub path: PathBuf,
    pub file: Mutex<Option<File>>,
}

#[atomic_delegates]
impl TestLog {
    #[must_use]
    pub fn open(test_name: &str) -> Self {}
    pub fn line(&self, message: &str) {}
}

#[cfg(test)]
mod tests {
    use super::TestLog;

    #[test]
    fn test_usage() {
        let log = TestLog::open("test_log_delegate_smoke");
        log.line("delegate smoke");
        let name = log.path.to_string_lossy();
        assert!(name.contains("test_log_delegate_smoke-"));
        assert!(name.ends_with(".log"));
        let _ = std::fs::remove_file(&log.path);
    }
}
