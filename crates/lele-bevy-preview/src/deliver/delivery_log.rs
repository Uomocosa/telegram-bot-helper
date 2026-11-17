use std::path::PathBuf;

use atomic_delegate_macros::atomic_delegates;

use crate::Error;
use crate::deliver;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeliveryLog {
    pub path: PathBuf,
    pub run_id: String,
    pub scene: String,
}

#[atomic_delegates]
impl DeliveryLog {
    #[must_use]
    pub fn from_out_dir_and_scene(out_dir: &std::path::Path, scene: &str) -> Self {}
}

#[atomic_delegates]
impl DeliveryLog {
    pub fn record_decision(
        &self,
        capture: &deliver::Capture,
        reason: deliver::Reason,
        diff: Option<&deliver::PixelDiff>,
    ) {
    }
    pub fn record_send(
        &self,
        chunk_index: usize,
        items: &[&deliver::Capture],
        resend_all: bool,
        result: &Result<usize, Error>,
    ) {
    }
    pub fn record_sent_unchanged(&self, capture: &deliver::Capture, why: &str) {}
}

#[rustfmt::skip]
impl DeliveryLog {
    pub fn record_skip(&self, why: &str) { self.append_line("skip", &[("why", why.to_string())]); }
}

#[atomic_delegates]
impl DeliveryLog {
    pub(crate) fn append_line(&self, event: &str, fields: &[(&str, String)]) {}
    pub(crate) fn utc_timestamp() -> String {}
    pub(crate) fn days_to_ymd(days: u64) -> (i64, u32, u32) {}
    #[must_use]
    pub(crate) fn sanitized_field(value: &str) -> String {}
    #[must_use]
    pub(crate) fn boxes_word(diff: &deliver::PixelDiff) -> String {}
}

#[cfg(test)]
mod tests {
    use super::DeliveryLog;

    #[test]
    fn test_usage() {
        let dir = tempfile::tempdir().unwrap();
        let log = DeliveryLog::from_out_dir_and_scene(dir.path(), "ui");
        assert!(log.path.ends_with("deliveries.log"));
        assert_eq!(log.run_id.len(), 8);
        assert_eq!(log.scene, "ui");
        log.record_skip("dry_run");
        let text = std::fs::read_to_string(&log.path).unwrap();
        assert!(text.contains("event=skip"));
        assert!(text.contains("why=dry_run"));
    }
}
