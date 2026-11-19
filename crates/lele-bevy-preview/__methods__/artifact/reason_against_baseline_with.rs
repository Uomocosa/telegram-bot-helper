use std::path::Path;

use crate::Error;
use crate::deliver;

pub fn reason_against_baseline_with(
    artifact: &deliver::Artifact,
    out_dir: &Path,
    new_image_path: &Path,
    drift: &deliver::DriftTolerance,
    is_exact: bool,
) -> Result<deliver::Reason, Error> {
    let delivered = artifact.delivered_path(out_dir);
    if !delivered.exists() {
        return Ok(deliver::Reason::NoBaseline);
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
        return Ok(deliver::Reason::PixelHashEqual);
    }
    if is_exact {
        return Ok(deliver::Reason::ExactModeDiffers);
    }
    let baseline_image = image::open(&delivered).map_err(|source| Error::Image {
        path: delivered.clone(),
        source,
    })?;
    let new_image = image::open(new_image_path).map_err(|source| Error::Image {
        path: new_image_path.to_path_buf(),
        source,
    })?;
    Ok(drift.reason_against_images(&baseline_image.to_rgba8(), &new_image.to_rgba8()))
}

#[cfg(test)]
mod tests {
    use super::reason_against_baseline_with;
    use crate::deliver;
    use crate::testing::canvas;
    use crate::testing::reason_for;
    use crate::testing::staged_artifact;
    use crate::testing::with_bordered_square;
    use crate::testing::with_horizontal_border;
    use crate::testing::with_square;
    use crate::testing::with_vertical_border;

    const LINE: image::Rgba<u8> = image::Rgba([250, 250, 250, 255]);
    const RED: image::Rgba<u8> = image::Rgba([200, 40, 40, 255]);
    const BLUE: image::Rgba<u8> = image::Rgba([40, 40, 200, 255]);

    #[test]
    fn test_same_pixels_are_pixel_hash_equal_in_both_modes() {
        let baseline = canvas(24, 24);
        assert_eq!(
            reason_for(&baseline, &baseline, false),
            deliver::Reason::PixelHashEqual
        );
        assert_eq!(
            reason_for(&baseline, &baseline, true),
            deliver::Reason::PixelHashEqual
        );
    }

    #[test]
    fn test_five_scattered_pixels_are_drift() {
        let baseline = canvas(32, 32);
        let mut new = baseline.clone();
        for (x, y) in [(2, 2), (10, 2), (18, 2), (2, 10), (10, 10)] {
            new.put_pixel(x, y, LINE);
        }
        assert_eq!(
            reason_for(&baseline, &new, false),
            deliver::Reason::DriftWithinTolerance
        );
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
        assert_eq!(
            reason_for(&baseline, &new, false),
            deliver::Reason::ClustersExceedDrift
        );
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
        assert_eq!(
            reason_for(&baseline, &new, false),
            deliver::Reason::ClustersExceedDrift
        );
    }

    #[test]
    fn test_different_size_is_changed() {
        let baseline = canvas(16, 16);
        let new = canvas(17, 16);
        assert_eq!(
            reason_for(&baseline, &new, false),
            deliver::Reason::SizeDiffers
        );
    }

    #[test]
    fn test_horizontal_border_moved_down_one_pixel_is_drift() {
        let baseline = with_horizontal_border(10);
        let new = with_horizontal_border(11);
        assert_eq!(
            reason_for(&baseline, &new, false),
            deliver::Reason::DriftWithinTolerance
        );
    }

    #[test]
    fn test_vertical_border_moved_sideways_one_pixel_is_drift() {
        let baseline = with_vertical_border(10);
        let new = with_vertical_border(11);
        assert_eq!(
            reason_for(&baseline, &new, false),
            deliver::Reason::DriftWithinTolerance
        );
    }

    #[test]
    fn test_vertical_border_moved_down_one_pixel_is_drift() {
        let mut old = canvas(24, 24);
        let mut new = old.clone();
        for y in 3..=20 {
            old.put_pixel(10, y, LINE);
            new.put_pixel(10, y.saturating_add(1), LINE);
        }
        assert_eq!(
            reason_for(&old, &new, false),
            deliver::Reason::DriftWithinTolerance
        );
    }

    #[test]
    fn test_element_moved_diagonally_one_pixel_is_drift() {
        let baseline = with_square(5, 5);
        let new = with_square(6, 6);
        assert_eq!(
            reason_for(&baseline, &new, false),
            deliver::Reason::DriftWithinTolerance
        );
    }

    #[test]
    fn test_new_divider_line_is_changed() {
        let baseline = canvas(24, 24);
        let mut new = baseline.clone();
        for y in 0..24 {
            new.put_pixel(12, y, LINE);
        }
        assert_eq!(
            reason_for(&baseline, &new, false),
            deliver::Reason::ClustersExceedDrift
        );
    }

    #[test]
    fn test_border_colour_changed_is_changed() {
        let baseline = with_bordered_square(RED);
        let new = with_bordered_square(BLUE);
        assert_eq!(
            reason_for(&baseline, &new, false),
            deliver::Reason::ClustersExceedDrift
        );
    }

    #[test]
    fn test_pill_moved_one_pixel_left_is_drift() {
        let baseline = with_square(10, 8);
        let new = with_square(9, 8);
        assert_eq!(
            reason_for(&baseline, &new, false),
            deliver::Reason::DriftWithinTolerance
        );
    }

    #[test]
    fn test_pill_moved_three_pixels_is_changed() {
        let baseline = with_square(10, 8);
        let new = with_square(7, 8);
        assert_eq!(
            reason_for(&baseline, &new, false),
            deliver::Reason::ClustersExceedDrift
        );
    }

    #[test]
    fn test_exact_flag_reports_a_shifted_image_as_changed() {
        let baseline = with_horizontal_border(10);
        let new = with_horizontal_border(11);
        assert_eq!(
            reason_for(&baseline, &new, true),
            deliver::Reason::ExactModeDiffers
        );
    }

    #[test]
    fn test_usage() {
        let dir = tempfile::tempdir().unwrap();
        let artifact = staged_artifact(&canvas(8, 8), dir.path());
        let new_path = dir.path().join("staged.png");
        assert_eq!(
            reason_against_baseline_with(
                &artifact,
                dir.path(),
                &new_path,
                &deliver::DriftTolerance::default(),
                false,
            )
            .unwrap(),
            deliver::Reason::NoBaseline
        );
    }
}
