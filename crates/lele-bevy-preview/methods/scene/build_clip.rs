use std::path::{Path, PathBuf};
use std::process::Command;

use crate::Error;
use crate::deliver;
use crate::deliver::basic::enums::Media;
use crate::preview;
use crate::scene;

const DEFAULT_FFMPEG: &str = "ffmpeg";

pub fn build_clip(
    scene: &scene::Scene,
    config: &preview::Config,
    previous: Option<&deliver::Manifest>,
) -> Result<Option<deliver::Capture>, Error> {
    let Some(spec) = scene.timeline.as_ref() else {
        return Ok(None);
    };
    let rendered = scene.render_frames(config)?;
    let mut frames = Vec::with_capacity(rendered.len());
    let mut hashes = Vec::with_capacity(rendered.len());
    for frame in rendered {
        frames.push(frame.path);
        hashes.push(frame.pixel_hash);
    }
    if hashes.windows(2).all(|pair| pair.first() == pair.get(1)) {
        return Err(Error::StaticClip {
            scene: scene.name.clone(),
            label: spec.label.clone(),
        });
    }
    let out = config.out_dir.join(format!(
        "{}__{}.mp4",
        scene.name,
        deliver::Artifact::slug(&spec.label)
    ));
    ffmpeg_build(&frames, spec.fps, &out, &scene.scratch_dir(config))?;
    let artifact = deliver::Artifact {
        scene: scene.name.clone(),
        label: spec.label.clone(),
        fingerprint: deliver::Artifact::fingerprint_of(
            &scene.name,
            &format!("clip:{}", spec.label),
        ),
        pixel_hash: clip_hash(spec.fps, &hashes),
        media: Media::Mp4,
        kind: scene.kind,
    };
    let status = artifact.status_in(previous);
    Ok(Some(deliver::Capture {
        path: out,
        artifact,
        status,
    }))
}

// needed helper: ffmpeg turns the scratch frames into the mp4 the manifest records
fn ffmpeg_build(
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
    let status = Command::new(ffmpeg_binary())
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

// needed helper: nix store paths are not on PATH, so LELE_FFMPEG overrides the binary name
fn ffmpeg_binary() -> String {
    match std::env::var("LELE_FFMPEG") {
        Ok(path) if !path.trim().is_empty() => path,
        _ => String::from(DEFAULT_FFMPEG),
    }
}

// needed helper: content hash over the clip's source frames so a no-op re-encode stays Same
fn clip_hash(fps: u32, hashes: &[String]) -> String {
    let joined = format!("{fps}:{}", hashes.join(","));
    deliver::Artifact::hash_bytes(joined.as_bytes())
}

#[cfg(test)]
mod tests {
    use super::build_clip;

    #[test]
    fn test_usage() {
        let _ = build_clip;
    }
}
