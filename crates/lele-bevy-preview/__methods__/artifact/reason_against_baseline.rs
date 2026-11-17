use std::path::Path;

use crate::Error;
use crate::deliver;

pub fn reason_against_baseline(
    artifact: &deliver::Artifact,
    out_dir: &Path,
    new_image_path: &Path,
    drift: &deliver::DriftTolerance,
) -> Result<deliver::Reason, Error> {
    artifact.reason_against_baseline_with(
        out_dir,
        new_image_path,
        drift,
        deliver::Artifact::is_exact_comparison_requested(),
    )
}

#[cfg(test)]
mod tests {
    use super::reason_against_baseline;
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
            reason_against_baseline(
                &artifact,
                dir.path(),
                &dir.path().join("missing.png"),
                &deliver::DriftTolerance::default(),
            )
            .unwrap(),
            deliver::Reason::NoBaseline
        );
    }
}
