use crate::Error;
use crate::deliver;
use crate::preview;
use crate::scene;

pub fn build_clip(
    scene: &scene::Scene,
    config: &preview::Config,
    staging_root: &std::path::Path,
) -> Result<Option<deliver::Capture>, Error> {
    let Some(spec) = scene.timeline.as_ref() else {
        return Ok(None);
    };
    let rendered = scene.render_frames(config, staging_root)?;
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
    let out = staging_root.join(format!(
        "{}__{}.mp4",
        scene.name,
        deliver::Artifact::slug(&spec.label)
    ));
    let staging_frames = deliver::Artifact::staging_frames_dir(staging_root, &scene.name);
    scene::Scene::ffmpeg_encode(&frames, spec.fps, &out, &staging_frames)?;
    let artifact = deliver::Artifact {
        scene: scene.name.clone(),
        label: spec.label.clone(),
        fingerprint: deliver::Artifact::fingerprint_of(
            &scene.name,
            &format!("clip:{}", spec.label),
        ),
        pixel_hash: scene::Scene::clip_hash(spec.fps, &hashes),
        media: deliver::Media::Mp4,
        kind: scene.kind,
    };
    let status = artifact.clip_status_against_baseline(&config.out_dir, spec.fps)?;
    Ok(Some(deliver::Capture {
        path: out,
        artifact,
        status,
    }))
}

#[cfg(test)]
mod tests {
    use super::build_clip;

    #[test]
    fn test_usage() {
        let _ = build_clip;
    }
}
