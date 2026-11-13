use std::path::Path;

use crate::basic;

#[must_use]
pub fn unsent_frames_dir(out_dir: &Path, scene: &str) -> std::path::PathBuf {
    out_dir
        .join(basic::constants::UNSENT_DIR)
        .join(format!("{scene}_frames"))
}

#[cfg(test)]
mod tests {
    use super::unsent_frames_dir;
    use crate::basic;
    use std::path::Path;

    #[test]
    fn test_usage() {
        let out_dir = Path::new("out");
        assert_eq!(
            unsent_frames_dir(out_dir, "ui"),
            out_dir.join(basic::constants::UNSENT_DIR).join("ui_frames")
        );
    }
}
