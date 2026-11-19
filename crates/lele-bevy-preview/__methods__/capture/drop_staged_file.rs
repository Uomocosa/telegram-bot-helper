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
    use super::drop_staged_file;
    use crate::deliver;
    use crate::testing::capture;

    #[test]
    fn test_usage() {
        let dir = tempfile::tempdir().unwrap();
        let staging_root = dir.path().join("staging");
        std::fs::create_dir_all(&staging_root).unwrap();
        let png = capture(
            staging_root.join("ui__rest.png"),
            "rest",
            deliver::Media::Png,
            deliver::Status::Changed,
        );
        std::fs::write(&png.path, b"x").unwrap();
        drop_staged_file(&png, &staging_root, "ui");
        assert!(!png.path.exists());

        let mp4 = capture(
            staging_root.join("ui__rest.mp4"),
            "rest",
            deliver::Media::Mp4,
            deliver::Status::Changed,
        );
        std::fs::write(&mp4.path, b"x").unwrap();
        let frames = deliver::Artifact::staging_frames_dir(&staging_root, "ui");
        std::fs::create_dir_all(&frames).unwrap();
        drop_staged_file(&mp4, &staging_root, "ui");
        assert!(!mp4.path.exists());
        assert!(!frames.exists());
    }
}
