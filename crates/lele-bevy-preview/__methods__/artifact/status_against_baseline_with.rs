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
    let delivered = artifact.delivered_path(out_dir);
    if !delivered.exists() {
        return Ok(deliver::Status::New);
    }
    let baseline = deliver::Artifact::hash_png_pixels(&delivered).map_err(|error| match error {
        Error::Image { path, source } => Error::Image { path, source },
        Error::Io(io) => Error::DeliveredPreviewUnreadable {
            path: delivered.clone(),
            source: io,
        },
        other => other,
    })?;
    if baseline == artifact.pixel_hash {
        return Ok(deliver::Status::Same);
    }
    if is_exact {
        return Ok(deliver::Status::Changed);
    }
    let baseline_image = image::open(&delivered).map_err(|source| Error::Image {
        path: delivered.clone(),
        source,
    })?;
    let new_image = image::open(new_image_path).map_err(|source| Error::Image {
        path: new_image_path.to_path_buf(),
        source,
    })?;
    Ok(drift.status_against_images(&baseline_image.to_rgba8(), &new_image.to_rgba8()))
}

#[cfg(test)]
mod tests {
    use std::path::{Path, PathBuf};

    use super::status_against_baseline_with;
    use crate::deliver;
    use crate::scene;

    const BACKGROUND: image::Rgba<u8> = image::Rgba([10, 12, 16, 255]);
    const LINE: image::Rgba<u8> = image::Rgba([250, 250, 250, 255]);
    const RED: image::Rgba<u8> = image::Rgba([200, 40, 40, 255]);
    const BLUE: image::Rgba<u8> = image::Rgba([40, 40, 200, 255]);

    fn canvas(width: u32, height: u32) -> image::RgbaImage {
        image::RgbaImage::from_pixel(width, height, BACKGROUND)
    }

    fn artifact_and_staged(new: &image::RgbaImage, dir: &Path) -> (deliver::Artifact, PathBuf) {
        let new_path = dir.join("staged.png");
        new.save(&new_path).unwrap();
        let artifact = deliver::Artifact {
            scene: String::from("ui"),
            label: String::from("rest"),
            fingerprint: deliver::Artifact::fingerprint_of("ui", "rest"),
            pixel_hash: deliver::Artifact::hash_png_pixels(&new_path).unwrap(),
            media: deliver::Media::Png,
            kind: scene::Kind::System,
        };
        (artifact, new_path)
    }

    fn status_for(
        baseline: &image::RgbaImage,
        new: &image::RgbaImage,
        is_exact: bool,
    ) -> deliver::Status {
        let dir = tempfile::tempdir().unwrap();
        let (artifact, new_path) = artifact_and_staged(new, dir.path());
        baseline.save(artifact.delivered_path(dir.path())).unwrap();
        status_against_baseline_with(
            &artifact,
            dir.path(),
            &new_path,
            &deliver::DriftTolerance::default(),
            is_exact,
        )
        .unwrap()
    }

    fn with_horizontal_border(y: u32) -> image::RgbaImage {
        let mut image = canvas(24, 24);
        for x in 0..24 {
            image.put_pixel(x, y, LINE);
        }
        image
    }

    fn with_vertical_border(x: u32) -> image::RgbaImage {
        let mut image = canvas(24, 24);
        for y in 0..24 {
            image.put_pixel(x, y, LINE);
        }
        image
    }

    fn with_bordered_square(colour: image::Rgba<u8>) -> image::RgbaImage {
        let mut image = canvas(24, 24);
        for y in 0..24 {
            for x in 0..24 {
                if !(3..21).contains(&x) || !(3..21).contains(&y) {
                    image.put_pixel(x, y, colour);
                }
            }
        }
        image
    }

    fn with_square(x_start: u32, y_start: u32) -> image::RgbaImage {
        let mut image = canvas(24, 24);
        for y in y_start..y_start.saturating_add(10) {
            for x in x_start..x_start.saturating_add(10) {
                image.put_pixel(x, y, LINE);
            }
        }
        image
    }

    #[test]
    fn test_usage() {
        let baseline = canvas(24, 24);
        assert_eq!(
            status_for(&baseline, &baseline, false),
            deliver::Status::Same
        );
        assert_eq!(
            status_for(&baseline, &baseline, true),
            deliver::Status::Same
        );
    }

    #[test]
    fn test_five_scattered_pixels_are_drift() {
        let baseline = canvas(32, 32);
        let mut new = baseline.clone();
        for (x, y) in [(2, 2), (10, 2), (18, 2), (2, 10), (10, 10)] {
            new.put_pixel(x, y, LINE);
        }
        assert_eq!(status_for(&baseline, &new, false), deliver::Status::Same);
    }

    #[test]
    fn test_recoloured_icon_is_changed() {
        let mut baseline = canvas(32, 32);
        let mut new = canvas(32, 32);
        for y in 4..14 {
            for x in 4..14 {
                baseline.put_pixel(x, y, RED);
                new.put_pixel(x, y, BLUE);
            }
        }
        assert_eq!(status_for(&baseline, &new, false), deliver::Status::Changed);
    }

    #[test]
    fn test_two_small_changes_in_opposite_corners_are_changed() {
        let baseline = canvas(28, 28);
        let mut new = baseline.clone();
        for y in 2..5 {
            for x in 2..5 {
                new.put_pixel(x, y, LINE);
            }
        }
        for y in 20..23 {
            for x in 20..23 {
                new.put_pixel(x, y, RED);
            }
        }
        assert_eq!(status_for(&baseline, &new, false), deliver::Status::Changed);
    }

    #[test]
    fn test_different_size_is_changed() {
        let baseline = canvas(16, 16);
        let new = canvas(17, 16);
        assert_eq!(status_for(&baseline, &new, false), deliver::Status::Changed);
    }

    #[test]
    fn test_horizontal_border_moved_down_one_pixel_is_drift() {
        let baseline = with_horizontal_border(10);
        let new = with_horizontal_border(11);
        assert_eq!(status_for(&baseline, &new, false), deliver::Status::Same);
    }

    #[test]
    fn test_vertical_border_moved_sideways_one_pixel_is_drift() {
        let baseline = with_vertical_border(10);
        let new = with_vertical_border(11);
        assert_eq!(status_for(&baseline, &new, false), deliver::Status::Same);
    }

    #[test]
    fn test_vertical_border_moved_down_one_pixel_is_drift() {
        let mut old = canvas(24, 24);
        let mut new = old.clone();
        for y in 3..=20 {
            old.put_pixel(10, y, LINE);
            new.put_pixel(10, y.saturating_add(1), LINE);
        }
        assert_eq!(status_for(&old, &new, false), deliver::Status::Same);
    }

    #[test]
    fn test_element_moved_diagonally_one_pixel_is_drift() {
        let baseline = with_square(5, 5);
        let new = with_square(6, 6);
        assert_eq!(status_for(&baseline, &new, false), deliver::Status::Same);
    }

    #[test]
    fn test_new_divider_line_is_changed() {
        let baseline = canvas(24, 24);
        let mut new = baseline.clone();
        for y in 0..24 {
            new.put_pixel(12, y, LINE);
        }
        assert_eq!(status_for(&baseline, &new, false), deliver::Status::Changed);
    }

    #[test]
    fn test_border_colour_changed_is_changed() {
        let baseline = with_bordered_square(RED);
        let new = with_bordered_square(BLUE);
        assert_eq!(status_for(&baseline, &new, false), deliver::Status::Changed);
    }

    #[test]
    fn test_pill_moved_one_pixel_left_is_drift() {
        let baseline = with_square(10, 8);
        let new = with_square(9, 8);
        assert_eq!(status_for(&baseline, &new, false), deliver::Status::Same);
    }

    #[test]
    fn test_pill_moved_three_pixels_is_changed() {
        let baseline = with_square(10, 8);
        let new = with_square(7, 8);
        assert_eq!(status_for(&baseline, &new, false), deliver::Status::Changed);
    }

    #[test]
    fn test_exact_flag_reports_a_shifted_image_as_changed() {
        let baseline = with_horizontal_border(10);
        let new = with_horizontal_border(11);
        assert_eq!(status_for(&baseline, &new, true), deliver::Status::Changed);
    }
}
