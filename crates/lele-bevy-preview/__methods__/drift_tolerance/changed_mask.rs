use image::Pixel;

use crate::deliver;

#[must_use]
pub fn changed_mask(
    tolerance: &deliver::DriftTolerance,
    baseline: &image::RgbaImage,
    new: &image::RgbaImage,
) -> Vec<bool> {
    baseline
        .pixels()
        .zip(new.pixels())
        .map(|(old, current)| {
            old.channels()
                .iter()
                .zip(current.channels().iter())
                .any(|(left, right)| left.abs_diff(*right) > tolerance.channel_delta)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::changed_mask;
    use crate::deliver;

    #[test]
    fn test_usage() {
        let tolerance = deliver::DriftTolerance::default();
        let baseline = image::RgbaImage::from_pixel(4, 4, image::Rgba([10, 10, 10, 255]));
        let mut changed = baseline.clone();
        changed.put_pixel(0, 0, image::Rgba([20, 10, 10, 255]));
        let mask = changed_mask(&tolerance, &baseline, &changed);
        assert_eq!(mask.len(), 16);
        assert_eq!(mask.iter().filter(|is_changed| **is_changed).count(), 1);
        assert!(
            changed_mask(&tolerance, &baseline, &baseline)
                .iter()
                .all(|is_changed| !is_changed)
        );
    }
}
