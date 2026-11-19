use std::path::Path;

use crate::deliver;
use crate::scene;
use crate::testing;

pub fn write_delivered(
    dir: &Path,
    scene_name: &str,
    label: &str,
    red: u8,
    green: u8,
    blue: u8,
) -> deliver::Artifact {
    let artifact = deliver::Artifact {
        scene: scene_name.to_string(),
        label: label.to_string(),
        fingerprint: deliver::Artifact::fingerprint_of(scene_name, label),
        pixel_hash: String::new(),
        media: deliver::Media::Png,
        kind: scene::Kind::System,
    };
    let path = artifact.delivered_path(dir);
    let hash = testing::solid_png(&path, red, green, blue);
    deliver::Artifact {
        pixel_hash: hash,
        ..artifact
    }
}

#[cfg(test)]
mod tests {
    use super::write_delivered;
    use crate::deliver;

    #[test]
    fn test_usage() {
        let dir = tempfile::tempdir().unwrap();
        let artifact = write_delivered(dir.path(), "ui", "one", 1, 2, 3);
        assert_eq!(artifact.pixel_hash.len(), deliver::HASH_CHARS);
    }
}
