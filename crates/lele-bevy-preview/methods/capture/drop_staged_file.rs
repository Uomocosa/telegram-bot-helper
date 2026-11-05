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

    #[test]
    fn test_usage() {
        let _ = drop_staged_file;
    }
}
