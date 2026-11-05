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
    use super::clip_status_against_baseline;

    #[test]
    fn test_usage() {
        let _ = clip_status_against_baseline;
    }
}
