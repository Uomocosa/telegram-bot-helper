use std::path::Path;

use crate::deliver;
use crate::deliver::basic::structs::Manifest;

#[must_use]
pub fn keep_on_disk(mut manifest: Manifest, dir: &Path) -> Manifest {
    manifest
        .artifacts
        .retain(|_, artifact| dir.join(deliver::file_name::file_name(artifact)).exists());
    manifest
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use crate::deliver::basic::enums::Media;
    use crate::deliver::basic::structs::{Artifact, Manifest};
    use crate::deliver::keep_on_disk::keep_on_disk;
    use crate::scene;

    // needed helper: fixture shared by the tests in this file
    fn artifact(label: &str) -> Artifact {
        Artifact {
            scene: String::from("ui"),
            label: label.to_string(),
            fingerprint: format!("ui|{label}"),
            pixel_hash: String::from("h"),
            media: Media::Png,
            kind: scene::Kind::System,
        }
    }

    #[test]
    fn test_usage() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join("ui__kept.png"), b"png").unwrap();
        let manifest = Manifest {
            crate_name: String::from("demo"),
            artifacts: BTreeMap::from([
                (String::from("ui|kept"), artifact("kept")),
                (String::from("ui|deleted"), artifact("deleted")),
            ]),
        };
        let kept = keep_on_disk(manifest, dir.path());
        assert_eq!(kept.artifacts.len(), 1);
        assert!(kept.artifacts.contains_key("ui|kept"));
    }
}
