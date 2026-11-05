use std::path::Path;

use crate::Error;
use crate::deliver;

pub fn promote_staged_to_delivered(
    capture: &mut deliver::Capture,
    out_dir: &Path,
    scene_name: &str,
    staging_root: &Path,
    resend_all: bool,
) -> Result<(), Error> {
    if capture.status == deliver::Status::Same && !resend_all {
        capture.drop_staged_file(staging_root, scene_name);
        let delivered = capture.artifact.delivered_path(out_dir);
        capture.path.clone_from(&delivered);
        capture.clear_unsent_entry(out_dir, scene_name);
        return Ok(());
    }
    if capture.is_mp4() {
        let staging_mp4 = capture.path.clone();
        let staging_frames = deliver::Artifact::staging_frames_dir(staging_root, scene_name);
        let delivered_mp4 = capture.artifact.delivered_path(out_dir);
        if let Some(parent) = delivered_mp4.parent() {
            std::fs::create_dir_all(parent).map_err(Error::Io)?;
        }
        std::fs::rename(&staging_mp4, &delivered_mp4).map_err(Error::Io)?;
        let delivered_frames = deliver::Artifact::delivered_frames_dir(out_dir, scene_name);
        let _ = std::fs::remove_dir_all(&delivered_frames);
        if staging_frames.exists() {
            if let Some(parent) = delivered_frames.parent() {
                std::fs::create_dir_all(parent).map_err(Error::Io)?;
            }
            std::fs::rename(&staging_frames, &delivered_frames).map_err(Error::Io)?;
        }
        let unsent_mp4 = capture.artifact.unsent_path(out_dir);
        let _ = std::fs::remove_file(&unsent_mp4);
        let unsent_frames = deliver::Artifact::unsent_frames_dir(out_dir, scene_name);
        let _ = std::fs::remove_dir_all(&unsent_frames);
        capture.path.clone_from(&delivered_mp4);
    } else {
        let staging = capture.path.clone();
        let delivered = capture.artifact.delivered_path(out_dir);
        if let Some(parent) = delivered.parent() {
            std::fs::create_dir_all(parent).map_err(Error::Io)?;
        }
        std::fs::rename(&staging, &delivered).map_err(Error::Io)?;
        let unsent = capture.artifact.unsent_path(out_dir);
        let _ = std::fs::remove_file(&unsent);
        capture.path.clone_from(&delivered);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::promote_staged_to_delivered;

    #[test]
    fn test_usage() {
        let _ = promote_staged_to_delivered;
    }
}
