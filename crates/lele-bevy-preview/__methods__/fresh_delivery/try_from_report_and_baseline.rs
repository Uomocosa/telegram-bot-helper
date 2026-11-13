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
    use std::path::{Path, PathBuf};

    use super::try_from_report_and_baseline;
    use crate::deliver;
    use crate::report::Report;
    use crate::scene;

    fn artifact(scene: &str, label: &str, pixel_hash: &str) -> deliver::Artifact {
        deliver::Artifact {
            scene: String::from(scene),
            label: String::from(label),
            fingerprint: deliver::Artifact::fingerprint_of(scene, label),
            pixel_hash: String::from(pixel_hash),
            media: deliver::Media::Png,
            kind: scene::Kind::System,
        }
    }

    fn write_solid(path: &Path, red: u8) -> String {
        image::RgbaImage::from_pixel(4, 4, image::Rgba([red, 0, 0, 255]))
            .save(path)
            .unwrap();
        deliver::Artifact::hash_png_pixels(path).unwrap()
    }

    fn report(path: PathBuf, pixel_hash: &str) -> Report {
        Report {
            captures: vec![deliver::Capture {
                path,
                artifact: artifact("ui", "rest", pixel_hash),
                status: deliver::Status::Changed,
            }],
            clip: None,
        }
    }

    #[test]
    fn test_usage() {
        let dir = tempfile::tempdir().unwrap();
        let out: &Path = dir.path();
        let shot = PathBuf::from("staging/ui__rest.png");
        let fresh = try_from_report_and_baseline(&report(shot.clone(), "unused"), out, 30).unwrap();
        assert_eq!(fresh.captures.len(), 1);
        assert_eq!(fresh.captures[0].status, deliver::Status::New);
        assert_eq!(fresh.captures[0].path, shot);
        assert!(fresh.clip.is_none());

        let baseline = write_solid(&artifact("ui", "rest", "x").delivered_path(out), 40);
        let fresh = try_from_report_and_baseline(&report(shot, &baseline), out, 30).unwrap();
        assert_eq!(fresh.captures[0].status, deliver::Status::Same);
    }
}
