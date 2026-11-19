use std::path::Path;

use crate::basic;

pub fn no_staging_left(dir: &Path) {
    let staging = dir.join(basic::constants::STAGING_DIR);
    if staging.exists() {
        let entries: Vec<_> = std::fs::read_dir(&staging).unwrap().collect();
        assert!(entries.is_empty(), "staging not cleaned: {staging:?}");
    }
}

#[cfg(test)]
mod tests {
    use super::no_staging_left;
    use crate::basic;

    #[test]
    fn test_usage() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::create_dir_all(dir.path().join(basic::constants::STAGING_DIR)).unwrap();
        no_staging_left(dir.path());
        assert!(dir.path().join(basic::constants::STAGING_DIR).exists());
    }
}
