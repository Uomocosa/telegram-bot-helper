use std::path::Path;

use crate::Error;
use crate::deliver;
use crate::scene;

pub fn clip_status_against_baseline(
    artifact: &deliver::Artifact,
    out_dir: &Path,
    fps: u32,
) -> Result<deliver::Status, Error> {
    let delivered = artifact.delivered_path(out_dir);
    if !delivered.exists() {
        return Ok(deliver::Status::New);
    }
    let frames_dir = deliver::Artifact::delivered_frames_dir(out_dir, &artifact.scene);
    if !frames_dir.is_dir() {
        return Ok(deliver::Status::New);
    }
    let mut names: Vec<std::ffi::OsString> = std::fs::read_dir(&frames_dir)
        .map_err(|source| Error::DeliveredPreviewUnreadable {
            path: frames_dir.clone(),
            source,
        })?
        .map(|entry| {
            entry
                .map_err(|source| Error::DeliveredPreviewUnreadable {
                    path: frames_dir.clone(),
                    source,
                })
                .map(|entry| entry.file_name())
        })
        .collect::<Result<Vec<std::ffi::OsString>, Error>>()?;
    names.sort();
    if names.is_empty() {
        return Ok(deliver::Status::New);
    }
    let mut frame_hashes = Vec::with_capacity(names.len());
    for name in &names {
        let path = frames_dir.join(name);
        let hash = deliver::Artifact::hash_png_pixels(&path)?;
        frame_hashes.push(hash);
    }
    let baseline = scene::Scene::clip_hash(fps, &frame_hashes);
    if baseline == artifact.pixel_hash {
        Ok(deliver::Status::Same)
    } else {
        Ok(deliver::Status::Changed)
    }
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
