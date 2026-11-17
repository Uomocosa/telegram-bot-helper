use image::Pixel;

use crate::deliver;

#[must_use]
pub fn does_shift_explain_cluster(
    tolerance: &deliver::DriftTolerance,
    cluster: &[(u32, u32)],
    baseline: &image::RgbaImage,
    new: &image::RgbaImage,
) -> bool {
    let mut min_x = u32::MAX;
    let mut min_y = u32::MAX;
    let mut max_x = 0;
    let mut max_y = 0;
    for (x, y) in cluster {
        min_x = min_x.min(*x);
        min_y = min_y.min(*y);
        max_x = max_x.max(*x);
        max_y = max_y.max(*y);
    }
    let width = new.width();
    let height = new.height();
    let last_x = width.saturating_sub(1);
    let last_y = height.saturating_sub(1);
    let window_min_x = min_x.saturating_sub(1);
    let window_min_y = min_y.saturating_sub(1);
    let window_max_x = max_x.saturating_add(1).min(last_x);
    let window_max_y = max_y.saturating_add(1).min(last_y);
    let max_shift = i64::from(tolerance.max_shift);
    let min_shift = max_shift.checked_neg().unwrap_or(0);
    for delta_y in min_shift..=max_shift {
        for delta_x in min_shift..=max_shift {
            if delta_x == 0 && delta_y == 0 {
                continue;
            }
            let mut does_every_pixel_match = true;
            for window_y in window_min_y..=window_max_y {
                for window_x in window_min_x..=window_max_x {
                    let source_x = i64::from(window_x).checked_sub(delta_x);
                    let source_y = i64::from(window_y).checked_sub(delta_y);
                    let (Some(Ok(source_x)), Some(Ok(source_y))) =
                        (source_x.map(u32::try_from), source_y.map(u32::try_from))
                    else {
                        does_every_pixel_match = false;
                        break;
                    };
                    if source_x >= width || source_y >= height {
                        does_every_pixel_match = false;
                        break;
                    }
                    let old = baseline.get_pixel(source_x, source_y);
                    let current = new.get_pixel(window_x, window_y);
                    let does_pixel_match = old
                        .channels()
                        .iter()
                        .zip(current.channels().iter())
                        .all(|(left, right)| left.abs_diff(*right) <= tolerance.channel_delta);
                    if !does_pixel_match {
                        does_every_pixel_match = false;
                        break;
                    }
                }
                if !does_every_pixel_match {
                    break;
                }
            }
            if does_every_pixel_match {
                return true;
            }
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::does_shift_explain_cluster;
    use crate::deliver;

    #[test]
    fn test_usage() {
        let tolerance = deliver::DriftTolerance::default();
        let mut baseline = image::RgbaImage::from_pixel(16, 16, image::Rgba([10, 12, 16, 255]));
        for x in 0..16 {
            baseline.put_pixel(x, 8, image::Rgba([250, 250, 250, 255]));
        }
        let mut new = baseline.clone();
        for x in 0..16 {
            new.put_pixel(x, 8, image::Rgba([10, 12, 16, 255]));
            new.put_pixel(x, 9, image::Rgba([250, 250, 250, 255]));
        }
        let cluster: Vec<(u32, u32)> = (0..16).flat_map(|x| [(x, 8), (x, 9)]).collect();
        assert!(does_shift_explain_cluster(
            &tolerance, &cluster, &baseline, &new
        ));
        let mut spot = image::RgbaImage::from_pixel(16, 16, image::Rgba([10, 12, 16, 255]));
        spot.put_pixel(5, 5, image::Rgba([250, 250, 250, 255]));
        assert!(!does_shift_explain_cluster(
            &tolerance,
            &[(5, 5)],
            &spot,
            &spot
        ));
    }
}
