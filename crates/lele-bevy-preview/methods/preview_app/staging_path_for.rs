use std::ffi::OsStr;
use std::path::{Path, PathBuf};

use crate::Error;

pub fn staging_path_for(out: &Path) -> Result<PathBuf, Error> {
    let parent = out.parent().map(Path::to_path_buf);
    if let Some(parent) = parent {
        std::fs::create_dir_all(&parent).map_err(Error::Io)?;
    }
    let extension = out.extension().and_then(OsStr::to_str).unwrap_or("png");
    let staging = out.with_extension(format!("{}.staging.{extension}", std::process::id()));
    let _ = std::fs::remove_file(&staging);
    Ok(staging)
}

#[cfg(test)]
mod tests {
    use super::staging_path_for;

    #[test]
    fn test_usage() {
        let dir = tempfile::tempdir().unwrap();
        let out = dir.path().join("nested").join("ui__rest.png");
        let staging = staging_path_for(&out).unwrap();
        assert!(dir.path().join("nested").is_dir());
        assert_ne!(staging, out);
        let name = staging.to_string_lossy().to_string();
        assert!(name.ends_with(&format!("ui__rest.{}.staging.png", std::process::id())));
    }
}
