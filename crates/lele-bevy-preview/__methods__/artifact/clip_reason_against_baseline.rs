use std::path::Path;

use crate::Error;
use crate::deliver;
use crate::scene;

pub fn clip_reason_against_baseline(
    artifact: &deliver::Artifact,
    out_dir: &Path,
    fps: u32,
) -> Result<deliver::Reason, Error> {
    let delivered = artifact.delivered_path(out_dir);
    if !delivered.exists() {
        return Ok(deliver::Reason::NoBaseline);
    }
    let frames_dir = deliver::Artifact::delivered_frames_dir(out_dir, &artifact.scene);
    if !frames_dir.is_dir() {
        return Ok(deliver::Reason::NoBaselineFrames);
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
        return Ok(deliver::Reason::NoBaselineFrames);
    }
    let mut frame_hashes = Vec::with_capacity(names.len());
    for name in &names {
        let path = frames_dir.join(name);
        let hash = deliver::Artifact::hash_png_pixels(&path)?;
        frame_hashes.push(hash);
    }
    let baseline = scene::Scene::clip_hash(fps, &frame_hashes);
    if baseline == artifact.pixel_hash {
        Ok(deliver::Reason::ClipHashEqual)
    } else {
        Ok(deliver::Reason::ClipHashDiffers)
    }
}

#[cfg(test)]
mod tests {
    use super::clip_reason_against_baseline;
    use crate::deliver;
    use crate::scene;
    use crate::testing::artifact;
    use crate::testing::solid_png;

    #[test]
    fn test_usage() {
        let dir = tempfile::tempdir().unwrap();
        let mut item = artifact("ui", "clip", "unused");
        item.media = deliver::Media::Mp4;
        assert_eq!(
            clip_reason_against_baseline(&item, dir.path(), 30).unwrap(),
            deliver::Reason::NoBaseline
        );
        std::fs::write(item.delivered_path(dir.path()), b"clip-bytes").unwrap();
        assert_eq!(
            clip_reason_against_baseline(&item, dir.path(), 30).unwrap(),
            deliver::Reason::NoBaselineFrames
        );
    }

    #[test]
    fn test_empty_frames_dir_is_no_baseline_frames() {
        let dir = tempfile::tempdir().unwrap();
        let item = artifact("ui", "clip", "unused");
        std::fs::write(item.delivered_path(dir.path()), b"clip-bytes").unwrap();
        std::fs::create_dir_all(deliver::Artifact::delivered_frames_dir(dir.path(), "ui")).unwrap();
        assert_eq!(
            clip_reason_against_baseline(&item, dir.path(), 30).unwrap(),
            deliver::Reason::NoBaselineFrames
        );
    }

    #[test]
    fn test_matching_frames_are_clip_hash_equal() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(
            artifact("ui", "clip", "x").delivered_path(dir.path()),
            b"clip-bytes",
        )
        .unwrap();
        let frames_dir = deliver::Artifact::delivered_frames_dir(dir.path(), "ui");
        std::fs::create_dir_all(&frames_dir).unwrap();
        let first = solid_png(&frames_dir.join("0001.png"), 10, 0, 0);
        let second = solid_png(&frames_dir.join("0002.png"), 20, 0, 0);
        let pixel_hash = scene::Scene::clip_hash(30, &[first, second]);
        let item = artifact("ui", "clip", &pixel_hash);
        assert_eq!(
            clip_reason_against_baseline(&item, dir.path(), 30).unwrap(),
            deliver::Reason::ClipHashEqual
        );
    }

    #[test]
    fn test_changed_frames_are_clip_hash_differs() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(
            artifact("ui", "clip", "x").delivered_path(dir.path()),
            b"clip-bytes",
        )
        .unwrap();
        let frames_dir = deliver::Artifact::delivered_frames_dir(dir.path(), "ui");
        std::fs::create_dir_all(&frames_dir).unwrap();
        solid_png(&frames_dir.join("0001.png"), 10, 0, 0);
        let item = artifact("ui", "clip", "stale-hash");
        assert_eq!(
            clip_reason_against_baseline(&item, dir.path(), 30).unwrap(),
            deliver::Reason::ClipHashDiffers
        );
    }
}
