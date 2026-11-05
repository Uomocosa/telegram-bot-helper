use crate::Error;
use crate::deliver;
use crate::report::Report;

pub fn try_from_report_and_baseline(
    report: &Report,
    out_dir: &std::path::Path,
    fps: u32,
) -> Result<deliver::FreshDelivery, Error> {
    let mut captures = Vec::new();
    for capture in &report.captures {
        let status = capture.artifact.status_against_baseline(out_dir)?;
        captures.push(deliver::Capture {
            path: capture.path.clone(),
            artifact: capture.artifact.clone(),
            status,
        });
    }
    let clip = report
        .clip
        .as_ref()
        .map(|previous| -> Result<deliver::Capture, Error> {
            let status = previous
                .artifact
                .clip_status_against_baseline(out_dir, fps)?;
            Ok(deliver::Capture {
                path: previous.path.clone(),
                artifact: previous.artifact.clone(),
                status,
            })
        })
        .transpose()?;
    Ok(deliver::FreshDelivery { captures, clip })
}

#[cfg(test)]
mod tests {
    use super::try_from_report_and_baseline;

    #[test]
    fn test_usage() {
        let _ = try_from_report_and_baseline;
    }
}
