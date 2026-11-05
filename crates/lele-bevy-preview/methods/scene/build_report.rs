use crate::Error;
use crate::deliver;
use crate::preview;
use crate::report::Report;
use crate::scene;

pub fn build_report(scene: &scene::Scene, config: &preview::Config) -> Result<Report, Error> {
    scene.check()?;
    let is_dry = std::env::var("LELE_PREVIEW_DRY_RUN").is_ok_and(|flag| flag == "1");
    if !is_dry {
        let _held = deliver::SentPreviewHash::acquire_scene_lock(&config.out_dir, &scene.name)?;
        for state in &scene.states {
            let preview = config.out_dir.join(format!(
                "{}__{}.png",
                scene.name,
                deliver::Artifact::slug(&state.label)
            ));
            if !preview.exists() {
                let artifact = deliver::Artifact {
                    scene: scene.name.clone(),
                    label: state.label.clone(),
                    fingerprint: deliver::Artifact::fingerprint_of(&scene.name, &state.label),
                    pixel_hash: String::new(),
                    media: deliver::Media::Png,
                    kind: scene.kind,
                };
                let sent = deliver::SentPreviewHash::sent_path(&config.out_dir, &artifact);
                let _ = std::fs::remove_file(&sent);
            }
        }
        if let Some(spec) = scene.timeline.as_ref() {
            let preview = config.out_dir.join(format!(
                "{}__{}.mp4",
                scene.name,
                deliver::Artifact::slug(&spec.label)
            ));
            if !preview.exists() {
                let artifact = deliver::Artifact {
                    scene: scene.name.clone(),
                    label: spec.label.clone(),
                    fingerprint: deliver::Artifact::fingerprint_of(
                        &scene.name,
                        &format!("clip:{}", spec.label),
                    ),
                    pixel_hash: String::new(),
                    media: deliver::Media::Mp4,
                    kind: scene.kind,
                };
                let sent = deliver::SentPreviewHash::sent_path(&config.out_dir, &artifact);
                let _ = std::fs::remove_file(&sent);
            }
        }
    }
    let rendered = scene.render_states(config)?;
    let captures = deliver::Capture::collect(&scene.name, scene.kind, &rendered, &config.out_dir)?;
    let clip = scene.build_clip(config)?;
    Ok(Report { captures, clip })
}

#[cfg(test)]
mod tests {
    use super::build_report;

    #[test]
    fn test_usage() {
        let _ = build_report;
    }
}
