use crate::Error;
use crate::deliver;
use crate::deliver::basic::enums::Media;
use crate::preview;
use crate::scene;

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
    scene::Scene::ffmpeg_encode(&frames, spec.fps, &out, &scene.scratch_dir(config))?;
    let artifact = deliver::Artifact {
        scene: scene.name.clone(),
        label: spec.label.clone(),
        fingerprint: deliver::Artifact::fingerprint_of(
            &scene.name,
            &format!("clip:{}", spec.label),
        ),
        pixel_hash: scene::Scene::clip_hash(spec.fps, &hashes),
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

#[cfg(test)]
mod tests {
    use super::build_clip;

    #[test]
    fn test_usage() {
        let _ = build_clip;
    }
}
