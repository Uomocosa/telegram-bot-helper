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
    use std::path::{Path, PathBuf};

    use super::assert_centered;
    use crate::Error;
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

    fn dot(path: &Path, x: u32, y: u32) {
        let mut image = image::RgbaImage::from_pixel(10, 10, image::Rgba([255, 255, 255, 255]));
        image.put_pixel(x, y, image::Rgba([0, 0, 0, 255]));
        image.save(path).unwrap();
    }

    fn capture(path: PathBuf) -> deliver::Capture {
        deliver::Capture {
            path,
            artifact: artifact("ui", "rest", "hash"),
            status: deliver::Status::New,
        }
    }

    #[test]
    fn test_usage() {
        let dir = tempfile::tempdir().unwrap();
        let centered: &Path = &dir.path().join("centered.png");
        dot(centered, 5, 5);
        let ok = Report {
            captures: vec![capture(centered.to_path_buf())],
            clip: None,
        };
        assert!(assert_centered(&ok, 0).is_ok());
        let off: &Path = &dir.path().join("off.png");
        dot(off, 1, 1);
        let bad = Report {
            captures: vec![capture(centered.to_path_buf()), capture(off.to_path_buf())],
            clip: None,
        };
        assert!(matches!(
            assert_centered(&bad, 0),
            Err(Error::NotCentered { x: 1, y: 1, .. })
        ));
    }
}
