use std::path::Path;

use crate::deliver;

pub fn clear_unsent_entry(capture: &deliver::Capture, out_dir: &Path, scene_name: &str) {
    let unsent = capture.artifact.unsent_path(out_dir);
    let _ = std::fs::remove_file(&unsent);
    if capture.is_mp4() {
        let unsent_frames = deliver::Artifact::unsent_frames_dir(out_dir, scene_name);
        let _ = std::fs::remove_dir_all(&unsent_frames);
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::clear_unsent_entry;
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

    fn capture(path: &str, media: deliver::Media) -> deliver::Capture {
        let mut item = artifact("ui", "rest", "hash");
        item.media = media;
        deliver::Capture {
            path: PathBuf::from(path),
            artifact: item,
            status: deliver::Status::Changed,
        }
    }

    #[test]
    fn test_usage() {
        let dir = tempfile::tempdir().unwrap();
        let png = capture("shot.png", deliver::Media::Png);
        let unsent = png.artifact.unsent_path(dir.path());
        std::fs::create_dir_all(unsent.parent().unwrap()).unwrap();
        std::fs::write(&unsent, b"x").unwrap();
        clear_unsent_entry(&png, dir.path(), "ui");
        assert!(!unsent.exists());

        let mp4 = capture("clip.mp4", deliver::Media::Mp4);
        let unsent_mp4 = mp4.artifact.unsent_path(dir.path());
        std::fs::create_dir_all(unsent_mp4.parent().unwrap()).unwrap();
        std::fs::write(&unsent_mp4, b"x").unwrap();
        let frames = deliver::Artifact::unsent_frames_dir(dir.path(), "ui");
        std::fs::create_dir_all(&frames).unwrap();
        clear_unsent_entry(&mp4, dir.path(), "ui");
        assert!(!unsent_mp4.exists());
        assert!(!frames.exists());
    }
}
