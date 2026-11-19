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
    let _frame = app.settled(out, config.max_capture_frames, config.min_distinct_colors)?;
    deliver::Artifact::hash_png_pixels(out)
}

#[cfg(test)]
mod tests {
    use crate::deliver;
    use crate::testing::fast_config;
    use crate::testing::scene_with;

    use super::render_state_in_fresh_app;

    #[test]
    fn test_usage() {
        let dir = tempfile::tempdir().unwrap();
        let scene = scene_with("ui", &["rest"], None);
        let out = dir.path().join("state.png");
        let hash =
            render_state_in_fresh_app(&scene, &fast_config(dir.path()), &scene.states[0], &out)
                .unwrap();
        assert_eq!(hash.len(), deliver::HASH_CHARS);
        assert!(out.exists());
    }
}
