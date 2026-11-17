use crate::deliver;

#[must_use]
pub fn reason_against_images(
    tolerance: &deliver::DriftTolerance,
    baseline: &image::RgbaImage,
    new: &image::RgbaImage,
) -> deliver::Reason {
    if baseline.dimensions() != new.dimensions() {
        return deliver::Reason::SizeDiffers;
    }
    let mask = tolerance.changed_mask(baseline, new);
    let clusters = tolerance.changed_clusters(&mask, new.width(), new.height());
    if clusters
        .iter()
        .all(|cluster| tolerance.is_cluster_drift(cluster, baseline, new))
    {
        deliver::Reason::DriftWithinTolerance
    } else {
        deliver::Reason::ClustersExceedDrift
    }
}

#[cfg(test)]
mod tests {
    use super::reason_against_images;
    use crate::deliver;

    #[test]
    fn test_usage() {
        let tolerance = deliver::DriftTolerance::default();
        let image = image::RgbaImage::from_pixel(8, 8, image::Rgba([10, 12, 16, 255]));
        assert_eq!(
            reason_against_images(&tolerance, &image, &image),
            deliver::Reason::DriftWithinTolerance
        );
        assert_eq!(
            reason_against_images(&tolerance, &image, &image).status(),
            deliver::Status::Same
        );
        let other = image::RgbaImage::from_pixel(9, 8, image::Rgba([10, 12, 16, 255]));
        assert_eq!(
            reason_against_images(&tolerance, &image, &other),
            deliver::Reason::SizeDiffers
        );
    }
}
