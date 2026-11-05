use std::path::Path;
use std::path::PathBuf;

use atomic_delegate_macros::atomic_delegates;
use serde::Deserialize;
use serde::Serialize;

use crate::Error;
use crate::deliver;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PreviewDecisionLog {
    pub time: String,
    pub run: String,
    pub pid: u32,
    pub test_scene: String,
    pub key: String,
    pub status: String,
    pub old_hash: String,
    pub new_hash: String,
    pub manifest: String,
    pub driver: String,
    pub outcome: String,
}

#[atomic_delegates]
impl PreviewDecisionLog {
    #[must_use]
    pub fn from_capture_old_hash_manifest_and_outcome(
        capture: &deliver::Capture,
        old_hash: &str,
        manifest: &str,
        outcome: &str,
    ) -> Self {
    }
    #[must_use]
    pub fn file_name() -> String {}
    #[must_use]
    pub fn log_path(out_dir: &Path) -> PathBuf {}
    pub fn append(&self, out_dir: &Path) -> Result<(), Error> {}
    #[must_use]
    pub(crate) fn current_time() -> String {}
    #[must_use]
    pub(crate) fn current_run() -> String {}
    #[must_use]
    pub(crate) fn current_driver() -> String {}
}

#[cfg(test)]
mod tests {
    use super::PreviewDecisionLog;
    use crate::deliver;
    use crate::scene;

    #[test]
    fn test_usage() {
        assert_ne!(PreviewDecisionLog::file_name().len(), 0);
        let dir = tempfile::tempdir().unwrap();
        let path = PreviewDecisionLog::log_path(dir.path());
        assert!(path.ends_with(PreviewDecisionLog::file_name()));
        assert_ne!(PreviewDecisionLog::current_time().len(), 0);
        assert_ne!(PreviewDecisionLog::current_run().len(), 0);
        assert_ne!(PreviewDecisionLog::current_driver().len(), 0);
        let capture = deliver::Capture {
            path: dir.path().join("ui__one.png"),
            artifact: deliver::Artifact {
                scene: String::from("ui"),
                label: String::from("one"),
                fingerprint: String::from("ui|one"),
                pixel_hash: String::from("h"),
                media: deliver::Media::Png,
                kind: scene::Kind::System,
            },
            status: deliver::Status::New,
        };
        let entry = PreviewDecisionLog::from_capture_old_hash_manifest_and_outcome(
            &capture, "", "missing", "sent",
        );
        assert_eq!(entry.test_scene, "ui");
        assert_eq!(entry.key, "ui|one");
        assert_eq!(entry.outcome, "sent");
        assert!(entry.append(dir.path()).is_ok());
        assert!(path.exists());
    }
}
