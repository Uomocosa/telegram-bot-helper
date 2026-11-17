use std::path::Path;

use crate::Error;
use crate::deliver;

pub fn status_against_baseline_with(
    artifact: &deliver::Artifact,
    out_dir: &Path,
    new_image_path: &Path,
    drift: &deliver::DriftTolerance,
    is_exact: bool,
) -> Result<deliver::Status, Error> {
    Ok(artifact
        .reason_against_baseline_with(out_dir, new_image_path, drift, is_exact)?
        .status())
}

#[cfg(test)]
mod tests {
    use super::status_against_baseline_with;
    use crate::deliver;
    use crate::scene;

    #[test]
    fn test_usage() {
        let dir = tempfile::tempdir().unwrap();
        let artifact = deliver::Artifact {
            scene: String::from("ui"),
            label: String::from("rest"),
            fingerprint: deliver::Artifact::fingerprint_of("ui", "rest"),
            pixel_hash: String::from("unused"),
            media: deliver::Media::Png,
            kind: scene::Kind::System,
        };
        assert_eq!(
            status_against_baseline_with(
                &artifact,
                dir.path(),
                &dir.path().join("missing.png"),
                &deliver::DriftTolerance::default(),
                false,
            )
            .unwrap(),
            deliver::Status::New
        );
    }
}
