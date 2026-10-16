use crate::Error;
use crate::Report;
use crate::layout;

pub fn assert_centered(report: &Report, tolerance: u32) -> Result<(), Error> {
    for capture in &report.captures {
        layout::assert_capture_centered::assert_capture_centered(capture, tolerance)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::assert_centered;
    use crate::Error;
    use crate::Report;
    use crate::deliver;

    fn report_of(path: &std::path::Path) -> Report {
        Report {
            captures: vec![deliver::Capture {
                path: path.to_path_buf(),
                artifact: deliver::Artifact {
                    scene: String::from("s"),
                    label: String::from("rest"),
                    fingerprint: String::from("s|rest"),
                    pixel_hash: String::new(),
                    media: deliver::Media::Png,
                    kind: crate::scene::Kind::System,
                },
                status: deliver::Status::New,
            }],
            clip: None,
        }
    }

    fn canvas_with_box(path: &std::path::Path, left: u32, top: u32) {
        let mut canvas = image::RgbaImage::from_pixel(40, 20, image::Rgba([0, 0, 0, 255]));
        for x in left..left.saturating_add(6) {
            for y in top..top.saturating_add(4) {
                canvas.put_pixel(x, y, image::Rgba([255, 255, 255, 255]));
            }
        }
        canvas.save(path).unwrap();
    }

    #[test]
    fn test_usage() {
        let dir = tempfile::tempdir().unwrap();
        let centred = dir.path().join("centred.png");
        canvas_with_box(&centred, 17, 8);
        assert!(assert_centered(&report_of(&centred), 1).is_ok());

        let corner = dir.path().join("corner.png");
        canvas_with_box(&corner, 0, 0);
        assert!(matches!(
            assert_centered(&report_of(&corner), 1),
            Err(Error::NotCentered { .. })
        ));
    }
}
