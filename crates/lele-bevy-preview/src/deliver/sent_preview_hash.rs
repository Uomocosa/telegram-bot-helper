use std::path::Path;
use std::path::PathBuf;

use atomic_delegate_macros::atomic_delegates;
use derive_more::Deref;

use crate::Error;
use crate::deliver;

#[derive(Debug, Clone, PartialEq, Eq, Deref)]
pub struct SentPreviewHash(pub String);

#[atomic_delegates]
impl SentPreviewHash {
    pub fn load_for(out_dir: &Path, artifact: &deliver::Artifact) -> Result<Option<Self>, Error> {}
    #[must_use]
    pub fn cache_dir(out_dir: &Path) -> PathBuf {}
    #[must_use]
    pub fn sent_path(out_dir: &Path, artifact: &deliver::Artifact) -> PathBuf {}
    pub fn record(out_dir: &Path, artifact: &deliver::Artifact) -> Result<(), Error> {}
    pub(crate) fn acquire_scene_lock(out_dir: &Path, scene: &str) -> Result<std::fs::File, Error> {}
}

#[cfg(test)]
mod tests {
    use super::SentPreviewHash;

    #[test]
    fn test_usage() {
        let dir = tempfile::tempdir().unwrap();
        let cache = SentPreviewHash::cache_dir(dir.path());
        assert!(cache.ends_with(crate::deliver::basic::constants::CACHE_DIR));
    }
}
