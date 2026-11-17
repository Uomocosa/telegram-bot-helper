use crate::deliver;

#[must_use]
pub fn is_cluster_drift(
    tolerance: &deliver::DriftTolerance,
    cluster: &[(u32, u32)],
    baseline: &image::RgbaImage,
    new: &image::RgbaImage,
) -> bool {
    if cluster.is_empty() {
        return true;
    }
    let pixel_count = u32::try_from(cluster.len()).unwrap_or(u32::MAX);
    if pixel_count < tolerance.min_pixels {
        return true;
    }
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
    let width = max_x.saturating_sub(min_x).saturating_add(1);
    let height = max_y.saturating_sub(min_y).saturating_add(1);
    let area = width.saturating_mul(height);
    let is_sparse = f64::from(pixel_count) < f64::from(tolerance.min_density) * f64::from(area);
    if is_sparse {
        return true;
    }
    tolerance.does_shift_explain_cluster(cluster, baseline, new)
}

#[cfg(test)]
mod tests {
    use super::is_cluster_drift;
    use crate::deliver;

    #[test]
    fn test_usage() {
        let tolerance = deliver::DriftTolerance::default();
        let baseline = image::RgbaImage::from_pixel(16, 16, image::Rgba([10, 12, 16, 255]));
        let mut new = baseline.clone();
        new.put_pixel(4, 4, image::Rgba([200, 200, 200, 255]));
        assert!(is_cluster_drift(&tolerance, &[(4, 4)], &baseline, &new));
        assert!(is_cluster_drift(&tolerance, &[], &baseline, &new));
    }
}
