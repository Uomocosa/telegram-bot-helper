use std::path::Path;

use crate::Error;
use crate::deliver;

pub fn clip_status_against_baseline(
    artifact: &deliver::Artifact,
    out_dir: &Path,
    fps: u32,
) -> Result<deliver::Status, Error> {
    Ok(artifact
        .clip_reason_against_baseline(out_dir, fps)?
        .status())
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::clip_status_against_baseline;
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
        let item = artifact("ui", "clip", "unused");
        assert_eq!(
            clip_status_against_baseline(&item, dir.path(), 30).unwrap(),
            deliver::Status::New
        );
        let _ = write_solid(&item.delivered_path(dir.path()), 10);
        assert_eq!(
            clip_status_against_baseline(&item, dir.path(), 30).unwrap(),
            deliver::Status::New
        );
    }
}
