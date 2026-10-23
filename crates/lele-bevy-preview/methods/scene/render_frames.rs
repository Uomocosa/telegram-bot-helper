use std::time::Duration;

use crate::Error;
use crate::PreviewApp;
use crate::deliver;
use crate::preview;
use crate::scene;
use crate::timeline;

pub fn render_frames(
    scene: &scene::Scene,
    config: &preview::Config,
) -> Result<Vec<timeline::RenderedFrame>, Error> {
    scene.check()?;
    let Some(spec) = scene.timeline.as_ref() else {
        return Ok(Vec::new());
    };
    let scratch = scene.scratch_dir(config);
    let _ = std::fs::remove_dir_all(&scratch);
    let mut app = PreviewApp::from_scene(scene, config);
    app.warmup();
    let mut frames = Vec::new();
    let total = config.lead_in_frames.saturating_add(spec.frames);
    let step = Duration::from_secs_f64(1.0 / f64::from(spec.fps));
    for index in 0..total {
        if index == config.lead_in_frames {
            (spec.apply)(app.app.world_mut());
        }
        app.advance(step);
        let path = scratch.join(format!("frame-{:04}.png", index.saturating_add(1)));
        let frame = app.capture(&path, config.max_capture_frames)?;
        frame.ensure_visual(&path, config.min_distinct_colors)?;
        frames.push(timeline::RenderedFrame {
            path,
            pixel_hash: deliver::Artifact::hash_bytes(frame.bytes()),
        });
    }
    Ok(frames)
}

#[cfg(test)]
mod tests {
    use super::render_frames;

    #[test]
    fn test_usage() {
        let _ = render_frames;
    }
}
