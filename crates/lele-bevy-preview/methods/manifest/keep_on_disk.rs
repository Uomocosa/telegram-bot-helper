use std::path::Path;

use crate::deliver;

#[must_use]
pub fn keep_on_disk(mut manifest: deliver::Manifest, dir: &Path) -> deliver::Manifest {
    manifest
        .artifacts
        .retain(|_, artifact| dir.join(artifact.file_name()).exists());
    manifest
}

#[cfg(test)]
mod tests {
    use super::keep_on_disk;

    #[test]
    fn test_usage() {
        let _ = keep_on_disk;
    }
}
