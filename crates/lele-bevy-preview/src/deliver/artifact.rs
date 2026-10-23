use std::path::Path;

use atomic_delegate_macros::atomic_delegates;
use serde::{Deserialize, Serialize};

use crate::Error;
use crate::deliver;
use crate::deliver::basic::enums::Media;
use crate::scene;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Artifact {
    pub scene: String,
    pub label: String,
    pub fingerprint: String,
    pub pixel_hash: String,
    #[serde(default)]
    pub media: Media,
    #[serde(default)]
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
    pub fn hash_file(path: &Path) -> Result<String, Error> {}
    #[must_use]
    pub fn status_in(&self, previous: Option<&deliver::Manifest>) -> deliver::Status {}
}

#[cfg(test)]
mod tests {
    use super::Artifact;
    use crate::deliver;
    use crate::deliver::basic::enums::Media;
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
        assert_eq!(artifact.status_in(None), deliver::Status::New);
        assert_eq!(artifact.fingerprint, "ui|one room");
        assert_ne!(Artifact::hash_bytes(b"bytes"), "");
    }
}
