use crate::deliver;

#[must_use]
pub fn pixel_diff(
    tolerance: &deliver::DriftTolerance,
    baseline: &image::RgbaImage,
    new: &image::RgbaImage,
) -> deliver::PixelDiff {
    if baseline.dimensions() != new.dimensions() {
        return deliver::PixelDiff {
            changed_pixels: 0,
            regions: Vec::new(),
        };
    }
    let mask = tolerance.changed_mask(baseline, new);
    let clusters = tolerance.changed_clusters(&mask, new.width(), new.height());
    let changed_pixels =
        u32::try_from(mask.iter().filter(|is_changed| **is_changed).count()).unwrap_or(u32::MAX);
    let regions = clusters
        .iter()
        .filter_map(|cluster| {
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
            let pixels = u32::try_from(cluster.len()).unwrap_or(u32::MAX);
            if min_x > max_x || min_y > max_y {
                return None;
            }
            Some(deliver::ChangedRegion {
                x: min_x,
                y: min_y,
                width: max_x.saturating_sub(min_x).saturating_add(1),
                height: max_y.saturating_sub(min_y).saturating_add(1),
                pixels,
                is_drift: tolerance.is_cluster_drift(cluster, baseline, new),
            })
        })
        .collect();
    deliver::PixelDiff {
        changed_pixels,
        regions,
    }
}

#[cfg(test)]
mod tests {
    use super::pixel_diff;
    use crate::deliver;

    const BACKGROUND: image::Rgba<u8> = image::Rgba([10, 12, 16, 255]);
    const LINE: image::Rgba<u8> = image::Rgba([250, 250, 250, 255]);
    const RED: image::Rgba<u8> = image::Rgba([200, 40, 40, 255]);
    const BLUE: image::Rgba<u8> = image::Rgba([40, 40, 200, 255]);

    #[test]
    fn test_usage() {
        let tolerance = deliver::DriftTolerance::default();
        let image = image::RgbaImage::from_pixel(8, 8, BACKGROUND);
        let diff = pixel_diff(&tolerance, &image, &image);
        assert_eq!(diff.changed_pixels, 0);
        assert_eq!(diff.regions.len(), 0);
        let other = image::RgbaImage::from_pixel(9, 8, BACKGROUND);
        let diff = pixel_diff(&tolerance, &image, &other);
        assert_eq!(diff.changed_pixels, 0);
        assert_eq!(diff.regions.len(), 0);
    }

    #[test]
    fn test_pixel_diff_reports_moved_and_real_regions() {
        let tolerance = deliver::DriftTolerance::default();
        let mut baseline = image::RgbaImage::from_pixel(24, 24, BACKGROUND);
        for x in 0..24 {
            baseline.put_pixel(x, 2, LINE);
        }
        for y in 10..20 {
            for x in 4..14 {
                baseline.put_pixel(x, y, RED);
            }
        }
        let mut new = image::RgbaImage::from_pixel(24, 24, BACKGROUND);
        for x in 0..24 {
            new.put_pixel(x, 3, LINE);
        }
        for y in 10..20 {
            for x in 4..14 {
                new.put_pixel(x, y, BLUE);
            }
        }
        let diff = pixel_diff(&tolerance, &baseline, &new);
        assert_eq!(diff.regions.len(), 2);
        let moved = diff
            .regions
            .iter()
            .find(|region| region.is_drift)
            .expect("shifted border is drift");
        assert_eq!(moved.y, 2);
        assert_eq!(moved.height, 2);
        let recoloured = diff
            .regions
            .iter()
            .find(|region| !region.is_drift)
            .expect("recoloured square is real");
        assert_eq!(recoloured.x, 4);
        assert_eq!(recoloured.y, 10);
        assert_eq!(recoloured.width, 10);
        assert_eq!(recoloured.height, 10);
        assert_eq!(recoloured.pixels, 100);
        assert_eq!(diff.changed_pixels, 148);
    }
}
