use std::path::Path;

use crate::basic;

#[must_use]
pub fn staging_root(out_dir: &Path, scene: &str) -> std::path::PathBuf {
    let thread = std::thread::current().id();
    out_dir
        .join(basic::constants::STAGING_DIR)
        .join(format!("{scene}.{}.{thread:?}", std::process::id()))
}

#[cfg(test)]
mod tests {
    use super::staging_root;

    #[test]
    fn test_usage() {
        let dir = tempfile::tempdir().unwrap();
        let root = staging_root(dir.path(), "ui");
        assert!(root.to_string_lossy().contains("_staging"));
        assert!(root.to_string_lossy().contains("ui."));
    }
}
