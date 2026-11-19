use std::path::Path;

use crate::Error;
use crate::deliver;

pub fn pixel_diff_against_baseline(
    artifact: &deliver::Artifact,
    out_dir: &Path,
    new_image_path: &Path,
    drift: &deliver::DriftTolerance,
) -> Result<Option<deliver::PixelDiff>, Error> {
    let delivered = artifact.delivered_path(out_dir);
    if !delivered.exists() {
        return Ok(None);
    }
    let baseline_image = image::open(&delivered).map_err(|source| Error::Image {
        path: delivered.clone(),
        source,
    })?;
    let new_image = image::open(new_image_path).map_err(|source| Error::Image {
        path: new_image_path.to_path_buf(),
        source,
    })?;
    Ok(Some(drift.pixel_diff(
        &baseline_image.to_rgba8(),
        &new_image.to_rgba8(),
    )))
}

#[cfg(test)]
mod tests {
    use super::pixel_diff_against_baseline;
    use crate::deliver;
    use crate::testing::artifact;

    #[test]
    fn test_usage() {
        let dir = tempfile::tempdir().unwrap();
        let missing = dir.path().join("missing.png");
        assert!(
            pixel_diff_against_baseline(
                &artifact("ui", "rest", "unused"),
                dir.path(),
                &missing,
                &deliver::DriftTolerance::default(),
            )
            .unwrap()
            .is_none()
        );
        let baseline = image::RgbaImage::from_pixel(4, 4, image::Rgba([10, 12, 16, 255]));
        let staged = dir.path().join("staged.png");
        baseline.save(&staged).unwrap();
        let delivered = artifact(
            "ui",
            "rest",
            &deliver::Artifact::hash_png_pixels(&staged).unwrap(),
        );
        baseline.save(delivered.delivered_path(dir.path())).unwrap();
        let diff = pixel_diff_against_baseline(
            &delivered,
            dir.path(),
            &staged,
            &deliver::DriftTolerance::default(),
        )
        .unwrap()
        .expect("delivered baseline gives a diff");
        assert_eq!(diff.changed_pixels, 0);
        assert_eq!(diff.regions.len(), 0);
    }
}
