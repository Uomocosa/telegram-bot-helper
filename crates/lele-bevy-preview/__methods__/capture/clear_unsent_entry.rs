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
    use super::clear_unsent_entry;
    use crate::deliver;
    use crate::testing::capture;

    #[test]
    fn test_usage() {
        let dir = tempfile::tempdir().unwrap();
        let png = capture(
            "shot.png",
            "rest",
            deliver::Media::Png,
            deliver::Status::Changed,
        );
        let unsent = png.artifact.unsent_path(dir.path());
        std::fs::create_dir_all(unsent.parent().unwrap()).unwrap();
        std::fs::write(&unsent, b"x").unwrap();
        clear_unsent_entry(&png, dir.path(), "ui");
        assert!(!unsent.exists());

        let mp4 = capture(
            "clip.mp4",
            "rest",
            deliver::Media::Mp4,
            deliver::Status::Changed,
        );
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
