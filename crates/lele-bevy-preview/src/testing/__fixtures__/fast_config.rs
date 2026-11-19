use std::path::Path;

use crate::preview;

pub fn fast_config(dir: &Path) -> preview::Config {
    preview::Config {
        out_dir: dir.to_path_buf(),
        warmup_frames: 5,
        lead_in_frames: 2,
        max_capture_frames: 240,
        min_distinct_colors: 1,
        ..preview::Config::default()
    }
}

#[cfg(test)]
mod tests {
    use super::fast_config;
    use std::path::Path;

    #[test]
    fn test_usage() {
        assert_eq!(fast_config(Path::new("out")).warmup_frames, 5);
    }
}
