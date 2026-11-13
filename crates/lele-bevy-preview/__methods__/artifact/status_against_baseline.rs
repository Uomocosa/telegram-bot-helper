use std::path::Path;

use crate::Error;
use crate::deliver;

pub fn status_against_baseline(
    artifact: &deliver::Artifact,
    out_dir: &Path,
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
        Ok(deliver::Status::Same)
    } else {
        Ok(deliver::Status::Changed)
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::status_against_baseline;
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

    fn write_solid(path: &Path, red: u8) -> String {
        image::RgbaImage::from_pixel(4, 4, image::Rgba([red, 0, 0, 255]))
            .save(path)
            .unwrap();
        deliver::Artifact::hash_png_pixels(path).unwrap()
    }

    #[test]
    fn test_usage() {
        let dir = tempfile::tempdir().unwrap();
        let item = artifact("ui", "rest", "unused");
        assert_eq!(
            status_against_baseline(&item, dir.path()).unwrap(),
            deliver::Status::New
        );
        let baseline = write_solid(&item.delivered_path(dir.path()), 200);
        let same = artifact("ui", "rest", &baseline);
        assert_eq!(
            status_against_baseline(&same, dir.path()).unwrap(),
            deliver::Status::Same
        );
        let other = artifact("ui", "rest", "other");
        assert_eq!(
            status_against_baseline(&other, dir.path()).unwrap(),
            deliver::Status::Changed
        );
    }
}
