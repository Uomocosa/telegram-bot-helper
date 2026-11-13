use std::path::Path;

use crate::deliver;

pub fn drop_staged_file(capture: &deliver::Capture, staging_root: &Path, scene_name: &str) {
    let _ = std::fs::remove_file(&capture.path);
    if capture.is_mp4() {
        let staging_frames = deliver::Artifact::staging_frames_dir(staging_root, scene_name);
        let _ = std::fs::remove_dir_all(&staging_frames);
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::drop_staged_file;
    use crate::deliver;
    use crate::scene;

    fn artifact(scene: &str, label: &str, pixel_hash: &str) -> deliver::Artifact {
        deliver::Artifact {
            scene: String::from(scene),
            label: String::from(label),
            fingerprint: deliver::Artifact::fingerprint_of(scene, label),
            pixel_hash: String::from(pixel_hash),
            media: deliver::Media::Png,
            kind: scene::Kind::System,
        }
    }

    fn capture(path: PathBuf, media: deliver::Media) -> deliver::Capture {
        let mut item = artifact("ui", "rest", "hash");
        item.media = media;
        deliver::Capture {
            path,
            artifact: item,
            status: deliver::Status::Changed,
        }
    }

    #[test]
    fn test_usage() {
        let dir = tempfile::tempdir().unwrap();
        let staging_root = dir.path().join("staging");
        std::fs::create_dir_all(&staging_root).unwrap();
        let png = capture(staging_root.join("ui__rest.png"), deliver::Media::Png);
        std::fs::write(&png.path, b"x").unwrap();
        drop_staged_file(&png, &staging_root, "ui");
        assert!(!png.path.exists());

        let mp4 = capture(staging_root.join("ui__rest.mp4"), deliver::Media::Mp4);
        std::fs::write(&mp4.path, b"x").unwrap();
        let frames = deliver::Artifact::staging_frames_dir(&staging_root, "ui");
        std::fs::create_dir_all(&frames).unwrap();
        drop_staged_file(&mp4, &staging_root, "ui");
        assert!(!mp4.path.exists());
        assert!(!frames.exists());
    }
}
