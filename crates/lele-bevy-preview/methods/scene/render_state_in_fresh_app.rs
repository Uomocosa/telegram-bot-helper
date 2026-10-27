use std::path::Path;

use crate::Error;
use crate::PreviewApp;
use crate::deliver;
use crate::preview;
use crate::scene;

pub fn render_state_in_fresh_app(
    scene: &scene::Scene,
    config: &preview::Config,
    state: &scene::State,
    out: &Path,
) -> Result<String, Error> {
    let mut app = PreviewApp::from_scene(scene, config);
    app.warmup();
    (state.apply)(app.app.world_mut());
    app.app.update();
    let frame = app.settled(out, config.max_capture_frames, config.min_distinct_colors)?;
    Ok(deliver::Artifact::hash_bytes(frame.bytes()))
}

#[cfg(test)]
mod tests {
    use super::render_state_in_fresh_app;

    #[test]
    fn test_usage() {
        let _ = render_state_in_fresh_app;
    }
}
