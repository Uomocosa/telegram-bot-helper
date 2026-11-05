use std::collections::BTreeMap;
use std::path::Path;

use atomic_delegate_macros::atomic_delegates;
use serde::{Deserialize, Serialize};

use crate::Error;
use crate::deliver;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Manifest {
    pub crate_name: String,
    pub artifacts: BTreeMap<String, deliver::Artifact>,
}

#[atomic_delegates]
impl Manifest {
    #[must_use]
    pub fn file_name() -> String {}
    pub fn load(dir: &Path) -> Result<Option<Manifest>, Error> {}
    #[must_use]
    pub fn keep_on_disk(self, dir: &Path) -> Manifest {}
    pub fn update(
        dir: &Path,
        crate_name: &str,
        recorded: &[&deliver::Capture],
        pending: &[&deliver::Capture],
    ) -> Result<(), Error> {
    }
    pub fn write_all(
        dir: &Path,
        crate_name: &str,
        captures: &[deliver::Capture],
    ) -> Result<(), Error> {
    }
    pub(crate) fn write_merged_atomically(dir: &Path, manifest: &Manifest) -> Result<(), Error> {}
    pub(crate) fn acquire_exclusive_lock(dir: &Path) -> Result<std::fs::File, Error> {}
    #[must_use]
    pub(crate) fn previous_hash_for(&self, artifact: &deliver::Artifact) -> String {}
}

#[cfg(test)]
mod tests {
    use super::Manifest;
    use crate::deliver;
    use crate::scene;
    use std::collections::BTreeMap;

    fn manifest() -> Manifest {
        Manifest {
            crate_name: String::from("demo"),
            artifacts: BTreeMap::from([(
                String::from("ui|gone"),
                deliver::Artifact {
                    scene: String::from("ui"),
                    label: String::from("gone"),
                    fingerprint: String::from("ui|gone"),
                    pixel_hash: String::from("h"),
                    media: deliver::Media::Png,
                    kind: scene::Kind::System,
                },
            )]),
        }
    }

    #[test]
    fn test_usage() {
        let dir = tempfile::tempdir().unwrap();
        assert!(
            std::path::Path::new(&Manifest::file_name())
                .extension()
                .is_some_and(|extension| extension.eq_ignore_ascii_case("json"))
        );
        assert!(Manifest::load(dir.path()).unwrap().is_none());
        let kept = manifest().keep_on_disk(dir.path());
        assert!(kept.artifacts.is_empty());
    }
}
