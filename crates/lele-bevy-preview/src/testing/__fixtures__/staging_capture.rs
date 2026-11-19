use std::path::Path;

use crate::deliver;
use crate::scene;
use crate::testing;

pub fn staging_capture(
    dir: &Path,
    scene_name: &str,
    label: &str,
    red: u8,
    green: u8,
    blue: u8,
    status: deliver::Status,
) -> deliver::Capture {
    let staging_root = deliver::Artifact::staging_root(dir, scene_name);
    std::fs::create_dir_all(&staging_root).unwrap();
    let probe = deliver::Artifact {
        scene: scene_name.to_string(),
        label: label.to_string(),
        fingerprint: deliver::Artifact::fingerprint_of(scene_name, label),
        pixel_hash: String::new(),
        media: deliver::Media::Png,
        kind: scene::Kind::System,
    };
    let file = probe.file_name();
    let staging = staging_root.join(file);
    let hash = testing::solid_png(&staging, red, green, blue);
    let artifact = deliver::Artifact {
        pixel_hash: hash,
        ..probe
    };
    deliver::Capture {
        path: staging,
        artifact,
        status,
    }
}

#[cfg(test)]
mod tests {
    use super::staging_capture;
    use crate::deliver;

    #[test]
    fn test_usage() {
        let dir = tempfile::tempdir().unwrap();
        let item = staging_capture(dir.path(), "ui", "one", 1, 2, 3, deliver::Status::New);
        assert!(item.path.exists());
    }
}
