use std::path::Path;

use atomic_delegate_macros::atomic_delegates;

use crate::Error;
use crate::basic;
use crate::deliver;
use crate::scene;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Artifact {
    pub scene: String,
    pub label: String,
    pub fingerprint: String,
    pub pixel_hash: String,
    pub media: deliver::Media,
    pub kind: scene::Kind,
}

#[atomic_delegates]
impl Artifact {
    #[must_use]
    pub fn slug(label: &str) -> String {}
    #[must_use]
    pub fn fingerprint_of(scene: &str, label: &str) -> String {}
    #[must_use]
    pub fn hash_bytes(bytes: &[u8]) -> String {}
    #[must_use]
    pub fn file_name(&self) -> String {}
    pub fn hash_png_pixels(path: &Path) -> Result<String, Error> {}
    #[must_use]
    pub fn staging_root(out_dir: &Path, scene: &str) -> std::path::PathBuf {}
    pub fn acquire_scene_lock(out_dir: &Path, scene: &str) -> Result<std::fs::File, Error> {}
    pub fn status_against_baseline(&self, out_dir: &Path) -> Result<deliver::Status, Error> {}
    pub fn clip_status_against_baseline(
        &self,
        out_dir: &Path,
        fps: u32,
    ) -> Result<deliver::Status, Error> {
    }
}

#[rustfmt::skip]
impl Artifact {
    #[must_use]
    pub fn delivered_path(&self, out_dir: &Path) -> std::path::PathBuf { out_dir.join(self.file_name()) }
    #[must_use]
    pub fn unsent_path(&self, out_dir: &Path) -> std::path::PathBuf { out_dir.join(basic::constants::UNSENT_DIR).join(self.file_name()) }
    #[must_use]
    pub fn locks_dir(out_dir: &Path) -> std::path::PathBuf { out_dir.join(basic::constants::LOCKS_DIR) }
    #[must_use]
    pub fn lock_path(out_dir: &Path, scene: &str) -> std::path::PathBuf { Self::locks_dir(out_dir).join(format!("{scene}.lock")) }
    #[must_use]
    pub fn delivered_frames_dir(out_dir: &Path, scene: &str) -> std::path::PathBuf { out_dir.join(format!("{scene}_frames")) }
    #[must_use]
    pub fn staging_frames_dir(staging_root: &Path, scene: &str) -> std::path::PathBuf { staging_root.join(format!("{scene}_frames")) }
    #[must_use]
    pub fn unsent_frames_dir(out_dir: &Path, scene: &str) -> std::path::PathBuf { out_dir.join(basic::constants::UNSENT_DIR).join(format!("{scene}_frames")) }
}

#[cfg(test)]
mod tests {
    use super::Artifact;
    use crate::deliver;
    use crate::deliver::Media;
    use crate::scene;

    fn artifact(label: &str, pixel_hash: &str) -> Artifact {
        Artifact {
            scene: String::from("ui"),
            label: String::from(label),
            fingerprint: Artifact::fingerprint_of("ui", label),
            pixel_hash: String::from(pixel_hash),
            media: Media::Png,
            kind: scene::Kind::System,
        }
    }

    #[test]
    fn test_usage() {
        let artifact = artifact("one room", "h1");
        assert_eq!(Artifact::slug("one room"), "one_room");
        assert_eq!(artifact.file_name(), "ui__one_room.png");
        let dir = tempfile::tempdir().unwrap();
        assert_eq!(
            artifact.status_against_baseline(dir.path()).unwrap(),
            deliver::Status::New
        );
        assert_eq!(artifact.fingerprint, "ui|one room");
        assert_ne!(Artifact::hash_bytes(b"bytes"), "");
    }
}
