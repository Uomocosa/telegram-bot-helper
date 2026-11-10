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
    use super::stash_staged_to_unsent;

    #[test]
    fn test_usage() {
        let _ = stash_staged_to_unsent;
    }
}
