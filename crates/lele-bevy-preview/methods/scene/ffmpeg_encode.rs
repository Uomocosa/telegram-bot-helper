use std::path::{Path, PathBuf};
use std::process::Command;

use crate::Error;
use crate::scene;

pub fn ffmpeg_encode(
    frames: &[PathBuf],
    fps: u32,
    out: &Path,
    scratch: &Path,
) -> Result<PathBuf, Error> {
    if frames.is_empty() {
        return Err(Error::Ffmpeg {
            path: out.display().to_string(),
            message: String::from("no frames were captured"),
        });
    }
    std::fs::create_dir_all(scratch).map_err(Error::Io)?;
    let destination = out.to_path_buf();
    let status = Command::new(scene::Scene::ffmpeg_binary())
        .arg("-y")
        .arg("-framerate")
        .arg(fps.to_string())
        .arg("-i")
        .arg(scratch.join("frame-%04d.png"))
        .arg("-c:v")
        .arg("libx264")
        .arg("-pix_fmt")
        .arg("yuv420p")
        .arg("-vf")
        .arg("pad=ceil(iw/2)*2:ceil(ih/2)*2")
        .arg("-r")
        .arg(fps.to_string())
        .arg(&destination)
        .status()
        .map_err(|source| Error::FfmpegSpawn {
            path: destination.display().to_string(),
            source,
        })?;
    if !status.success() {
        return Err(Error::Ffmpeg {
            path: destination.display().to_string(),
            message: format!("ffmpeg exited with {status}"),
        });
    }
    Ok(destination)
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::ffmpeg_encode;
    use crate::Error;

    #[test]
    fn test_usage() {
        let dir = tempfile::tempdir().unwrap();
        let out = dir.path().join("clip.mp4");
        let result = ffmpeg_encode(&[], 10, &out, Path::new("unused"));
        assert!(matches!(result, Err(Error::Ffmpeg { .. })));
    }
}
