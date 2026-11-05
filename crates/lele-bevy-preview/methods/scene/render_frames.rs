use std::time::Duration;

use crate::Error;
use crate::PreviewApp;
use crate::deliver;
use crate::preview;
use crate::scene;
use crate::timeline;

const TIMELINE_SETTLE_UPDATES: u32 = 5;

pub fn render_frames(
    scene: &scene::Scene,
    config: &preview::Config,
    staging_root: &std::path::Path,
) -> Result<Vec<timeline::RenderedFrame>, Error> {
    scene.check()?;
    let Some(spec) = scene.timeline.as_ref() else {
        return Ok(Vec::new());
    };
    let staging_frames = deliver::Artifact::staging_frames_dir(staging_root, &scene.name);
    let _ = std::fs::remove_dir_all(&staging_frames);
    std::fs::create_dir_all(&staging_frames).map_err(Error::Io)?;
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
        for _ in 0..TIMELINE_SETTLE_UPDATES {
            app.app.update();
        }
        let path = staging_frames.join(format!("frame-{:04}.png", index.saturating_add(1)));
        let frame = app.capture(&path, config.max_capture_frames)?;
        frame.ensure_visual(&path, config.min_distinct_colors)?;
        let pixel_hash = deliver::Artifact::hash_png_pixels(&path)?;
        frames.push(timeline::RenderedFrame { path, pixel_hash });
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
