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

    #[test]
    fn test_usage() {
        let _ = clear_unsent_entry;
    }
}
