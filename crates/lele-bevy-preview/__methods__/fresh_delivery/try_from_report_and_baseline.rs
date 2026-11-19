use crate::Error;
use crate::deliver;
use crate::report::Report;

pub fn try_from_report_and_baseline(
    report: &Report,
    out_dir: &std::path::Path,
    fps: u32,
    drift: &deliver::DriftTolerance,
) -> Result<deliver::FreshDelivery, Error> {
    let mut captures = Vec::new();
    let mut reasons = Vec::new();
    let mut diffs = Vec::new();
    for capture in &report.captures {
        let reason = capture
            .artifact
            .reason_against_baseline(out_dir, &capture.path, drift)?;
        let diff = if matches!(
            reason,
            deliver::Reason::DriftWithinTolerance | deliver::Reason::ClustersExceedDrift
        ) {
            capture
                .artifact
                .pixel_diff_against_baseline(out_dir, &capture.path, drift)?
        } else {
            None
        };
        captures.push(deliver::Capture {
            path: capture.path.clone(),
            artifact: capture.artifact.clone(),
            status: reason.status(),
        });
        reasons.push(reason);
        diffs.push(diff);
    }
    let (clip, clip_reason) = if let Some(previous) = report.clip.as_ref() {
        let reason = previous
            .artifact
            .clip_reason_against_baseline(out_dir, fps)?;
        (
            Some(deliver::Capture {
                path: previous.path.clone(),
                artifact: previous.artifact.clone(),
                status: reason.status(),
            }),
            Some(reason),
        )
    } else {
        (None, None)
    };
    Ok(deliver::FreshDelivery {
        captures,
        reasons,
        diffs,
        clip,
        clip_reason,
    })
}

#[cfg(test)]
mod tests {
    use std::path::{Path, PathBuf};

    use super::try_from_report_and_baseline;
    use crate::deliver;
    use crate::testing::artifact;
    use crate::testing::report;
    use crate::testing::solid_png;

    #[test]
    fn test_usage() {
        let dir = tempfile::tempdir().unwrap();
        let out: &Path = dir.path();
        let shot = PathBuf::from("staging/ui__rest.png");
        let fresh = try_from_report_and_baseline(
            &report(shot.clone(), "unused"),
            out,
            30,
            &deliver::DriftTolerance::default(),
        )
        .unwrap();
        assert_eq!(fresh.captures.len(), 1);
        assert_eq!(fresh.captures[0].status, deliver::Status::New);
        assert_eq!(fresh.reasons, vec![deliver::Reason::NoBaseline]);
        assert_eq!(fresh.diffs, vec![None]);
        assert_eq!(fresh.captures[0].path, shot);
        assert!(fresh.clip.is_none());
        assert!(fresh.clip_reason.is_none());

        let baseline = solid_png(&artifact("ui", "rest", "x").delivered_path(out), 40, 0, 0);
        let fresh = try_from_report_and_baseline(
            &report(shot, &baseline),
            out,
            30,
            &deliver::DriftTolerance::default(),
        )
        .unwrap();
        assert_eq!(fresh.captures[0].status, deliver::Status::Same);
        assert_eq!(fresh.reasons, vec![deliver::Reason::PixelHashEqual]);
        assert_eq!(fresh.diffs, vec![None]);
    }
}
