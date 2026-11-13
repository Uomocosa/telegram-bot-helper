use std::path::Path;

use crate::Error;
use crate::deliver;

pub fn stash_staged_to_unsent(
    capture: &mut deliver::Capture,
    out_dir: &Path,
    scene_name: &str,
    staging_root: &Path,
) -> Result<(), Error> {
    if capture.status == deliver::Status::Same {
        capture.drop_staged_file(staging_root, scene_name);
        let delivered = capture.artifact.delivered_path(out_dir);
        capture.path.clone_from(&delivered);
        capture.clear_unsent_entry(out_dir, scene_name);
        return Ok(());
    }
    if capture.is_mp4() {
        let staging_mp4 = capture.path.clone();
        let staging_frames = deliver::Artifact::staging_frames_dir(staging_root, scene_name);
        let unsent_mp4 = capture.artifact.unsent_path(out_dir);
        if let Some(parent) = unsent_mp4.parent() {
            std::fs::create_dir_all(parent).map_err(Error::Io)?;
        }
        let _ = std::fs::remove_file(&unsent_mp4);
        if staging_mp4.exists() {
            std::fs::rename(&staging_mp4, &unsent_mp4).map_err(Error::Io)?;
        }
        let unsent_frames = deliver::Artifact::unsent_frames_dir(out_dir, scene_name);
        let _ = std::fs::remove_dir_all(&unsent_frames);
        if staging_frames.exists() {
            if let Some(parent) = unsent_frames.parent() {
                std::fs::create_dir_all(parent).map_err(Error::Io)?;
            }
            std::fs::rename(&staging_frames, &unsent_frames).map_err(Error::Io)?;
        }
        capture.path.clone_from(&unsent_mp4);
    } else {
        let staging = capture.path.clone();
        let unsent = capture.artifact.unsent_path(out_dir);
        if let Some(parent) = unsent.parent() {
            std::fs::create_dir_all(parent).map_err(Error::Io)?;
        }
        let _ = std::fs::remove_file(&unsent);
        if staging.exists() {
            std::fs::rename(&staging, &unsent).map_err(Error::Io)?;
        }
        capture.path.clone_from(&unsent);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::stash_staged_to_unsent;
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

    fn capture(path: PathBuf, status: deliver::Status) -> deliver::Capture {
        deliver::Capture {
            path,
            artifact: artifact("ui", "rest", "hash"),
            status,
        }
    }

    #[test]
    fn test_usage() {
        let dir = tempfile::tempdir().unwrap();
        let out = dir.path().join("out");
        let staging = dir.path().join("staging");
        std::fs::create_dir_all(&staging).unwrap();

        let mut changed = capture(staging.join("ui__rest.png"), deliver::Status::Changed);
        std::fs::write(&changed.path, b"changed").unwrap();
        stash_staged_to_unsent(&mut changed, &out, "ui", &staging).unwrap();
        let unsent = changed.artifact.unsent_path(&out);
        assert!(unsent.is_file());
        assert_eq!(changed.path, unsent);
        assert!(!staging.join("ui__rest.png").exists());

        let mut same = capture(staging.join("ui__rest.png"), deliver::Status::Same);
        std::fs::write(&same.path, b"same").unwrap();
        stash_staged_to_unsent(&mut same, &out, "ui", &staging).unwrap();
        assert_eq!(same.path, same.artifact.delivered_path(&out));
        assert!(!staging.join("ui__rest.png").exists());
        assert!(!same.artifact.delivered_path(&out).exists());
    }
}
