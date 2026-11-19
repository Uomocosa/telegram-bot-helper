use crate::deliver;
use crate::testing;

pub fn reason_for(
    baseline: &image::RgbaImage,
    new: &image::RgbaImage,
    is_exact: bool,
) -> deliver::Reason {
    let dir = tempfile::tempdir().unwrap();
    let artifact = testing::staged_artifact(new, dir.path());
    let new_path = dir.path().join("staged.png");
    baseline.save(artifact.delivered_path(dir.path())).unwrap();
    artifact
        .reason_against_baseline_with(
            dir.path(),
            &new_path,
            &deliver::DriftTolerance::default(),
            is_exact,
        )
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::reason_for;
    use crate::deliver;
    use crate::testing::canvas;

    #[test]
    fn test_usage() {
        let baseline = canvas(8, 8);
        assert_eq!(
            reason_for(&baseline, &baseline, false),
            deliver::Reason::PixelHashEqual
        );
    }
}
