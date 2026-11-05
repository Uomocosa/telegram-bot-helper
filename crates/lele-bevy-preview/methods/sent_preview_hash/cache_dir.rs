use std::path::Path;

use crate::deliver::basic::constants::CACHE_DIR;

#[must_use]
pub fn cache_dir(out_dir: &Path) -> std::path::PathBuf {
    let mut base = out_dir.to_path_buf();
    base.push(CACHE_DIR);
    base
}

#[cfg(test)]
mod tests {
    use super::cache_dir;

    #[test]
    fn test_usage() {
        let dir = tempfile::tempdir().unwrap();
        assert!(cache_dir(dir.path()).ends_with(crate::deliver::basic::constants::CACHE_DIR));
    }
}
