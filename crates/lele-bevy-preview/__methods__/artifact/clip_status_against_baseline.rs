use std::path::Path;

use crate::Error;
use crate::deliver;

pub fn clip_status_against_baseline(
    artifact: &deliver::Artifact,
    out_dir: &Path,
    fps: u32,
) -> Result<deliver::Status, Error> {
    Ok(artifact
        .clip_reason_against_baseline(out_dir, fps)?
        .status())
}

#[cfg(test)]
mod tests {
    use super::clip_status_against_baseline;
    use crate::deliver;
    use crate::testing::artifact;
    use crate::testing::solid_png;

    #[test]
    fn test_usage() {
        let dir = tempfile::tempdir().unwrap();
        let item = artifact("ui", "clip", "unused");
        assert_eq!(
            clip_status_against_baseline(&item, dir.path(), 30).unwrap(),
            deliver::Status::New
        );
        let _ = solid_png(&item.delivered_path(dir.path()), 10, 0, 0);
        assert_eq!(
            clip_status_against_baseline(&item, dir.path(), 30).unwrap(),
            deliver::Status::New
        );
    }
}
