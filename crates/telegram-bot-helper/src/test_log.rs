use std::fs::File;
use std::io::Write as _;
use std::path::PathBuf;
use std::sync::Mutex;

use atomic_delegate_macros::atomic_delegates;

pub struct TestLog {
    pub path: PathBuf,
    pub file: Mutex<Option<File>>,
}

#[atomic_delegates]
impl From<&str> for TestLog {
    fn from(test_name: &str) -> Self {}
}

#[atomic_delegates]
impl TestLog {
    pub fn line(&self, message: &str) {}
}

#[rustfmt::skip]
impl TestLog {
    pub(crate) fn append_if_open(&self, stamped: &str) { if let Some(file) = self.file.lock().unwrap_or_else(std::sync::PoisonError::into_inner).as_mut() { let _ = writeln!(file, "{stamped}"); } }
}

#[cfg(test)]
mod tests {
    use super::TestLog;

    #[test]
    fn test_usage() {
        let log = TestLog::from("test_log_delegate_smoke");
        log.line("delegate smoke");
        let name = log.path.to_string_lossy();
        assert!(name.contains("test_log_delegate_smoke-"));
        assert!(name.ends_with(".log"));
        let _ = std::fs::remove_file(&log.path);
    }
}
