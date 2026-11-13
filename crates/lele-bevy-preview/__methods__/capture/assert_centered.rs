use crate::Error;
use crate::deliver;
use crate::layout;

pub fn assert_centered(capture: &deliver::Capture, tolerance: u32) -> Result<(), Error> {
    let path = capture.path.display().to_string();
    let bounds =
        layout::Bounds::of_image(&capture.path)?.ok_or_else(|| Error::NoContent(path.clone()))?;
    let x = bounds.min_x.saturating_add(bounds.max_x).div_ceil(2);
    let y = bounds.min_y.saturating_add(bounds.max_y).div_ceil(2);
    let expected_x = bounds.width / 2;
    let expected_y = bounds.height / 2;
    if x.abs_diff(expected_x) > tolerance || y.abs_diff(expected_y) > tolerance {
        return Err(Error::NotCentered {
            path,
            x,
            y,
            expected_x,
            expected_y,
            tolerance,
        });
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::path::{Path, PathBuf};

    use super::assert_centered;
    use crate::Error;
    use crate::deliver;
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
        let path = dir.path().join("shot.png");
        dot(&path, 5, 5);
        assert!(assert_centered(&capture(path.clone()), 0).is_ok());
        dot(&path, 1, 1);
        assert!(matches!(
            assert_centered(&capture(path), 0),
            Err(Error::NotCentered {
                x: 1,
                y: 1,
                expected_x: 5,
                expected_y: 5,
                tolerance: 0,
                ..
            })
        ));
    }
}
