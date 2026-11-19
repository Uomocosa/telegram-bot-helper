use crate::Error;
use crate::report::Report;

pub fn assert_centered(report: &Report, tolerance: u32) -> Result<(), Error> {
    for capture in &report.captures {
        capture.assert_centered(tolerance)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::assert_centered;
    use crate::Error;
    use crate::deliver;
    use crate::report::Report;
    use crate::testing::capture;
    use crate::testing::dot;

    #[test]
    fn test_usage() {
        let dir = tempfile::tempdir().unwrap();
        let centered: &Path = &dir.path().join("centered.png");
        dot(centered, 5, 5);
        let ok = Report {
            captures: vec![capture(
                centered.to_path_buf(),
                "rest",
                deliver::Media::Png,
                deliver::Status::New,
            )],
            clip: None,
        };
        assert!(assert_centered(&ok, 0).is_ok());
        let off: &Path = &dir.path().join("off.png");
        dot(off, 1, 1);
        let bad = Report {
            captures: vec![
                capture(
                    centered.to_path_buf(),
                    "rest",
                    deliver::Media::Png,
                    deliver::Status::New,
                ),
                capture(
                    off.to_path_buf(),
                    "rest",
                    deliver::Media::Png,
                    deliver::Status::New,
                ),
            ],
            clip: None,
        };
        assert!(matches!(
            assert_centered(&bad, 0),
            Err(Error::NotCentered { x: 1, y: 1, .. })
        ));
    }
}
