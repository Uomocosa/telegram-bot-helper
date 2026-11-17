use crate::deliver;

#[must_use]
pub fn status_against_images(
    tolerance: &deliver::DriftTolerance,
    baseline: &image::RgbaImage,
    new: &image::RgbaImage,
) -> deliver::Status {
    if baseline.dimensions() != new.dimensions() {
        return deliver::Status::Changed;
    }
    let mask = tolerance.changed_mask(baseline, new);
    let clusters = tolerance.changed_clusters(&mask, new.width(), new.height());
    if clusters
        .iter()
        .all(|cluster| tolerance.is_cluster_drift(cluster, baseline, new))
    {
        deliver::Status::Same
    } else {
        deliver::Status::Changed
    }
}

#[cfg(test)]
mod tests {
    use super::status_against_images;
    use crate::deliver;

    #[test]
    fn test_usage() {
        let tolerance = deliver::DriftTolerance::default();
        let image = image::RgbaImage::from_pixel(8, 8, image::Rgba([10, 12, 16, 255]));
        assert_eq!(
            status_against_images(&tolerance, &image, &image),
            deliver::Status::Same
        );
    }
}
