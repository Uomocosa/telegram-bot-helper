#[cfg(test)]
mod tests {
    use crate::PreviewApp;
    use crate::deliver;
    use crate::preview;
    use crate::scene;
    use crate::testing::fast_config;
    use crate::testing::scene_with;
    use bevy::prelude::*;

    #[test]
    fn test_usage() {
        let config = preview::Config::default();
        assert_eq!(config.width, preview::DEFAULT_WIDTH);
        assert_eq!(config.height, preview::DEFAULT_HEIGHT);
        assert_eq!(config.warmup_frames, preview::DEFAULT_WARMUP_FRAMES);
        assert!(config.out_dir.ends_with("ui_preview"));
    }

    #[test]
    fn test_usage_from_scene_produces_an_offscreen_target_and_a_camera() {
        let mut app = PreviewApp::from_scene(
            &scene_with("demo", &["one room", "no rooms!"], None),
            &preview::Config::default(),
        );
        assert_ne!(app.target.clone(), Handle::default());
        let world = app.app.world_mut();
        let mut cameras = world.query_filtered::<Entity, With<bevy::camera::RenderTarget>>();
        assert!(cameras.iter(world).count() > 0);
    }

    #[test]
    fn test_usage_capture_writes_a_non_empty_png_and_creates_parents() {
        let mut app = PreviewApp::from_scene(
            &scene_with("demo", &["one room", "no rooms!"], None),
            &preview::Config::default(),
        );
        let dir = tempfile::tempdir().unwrap();
        let out = dir.path().join("nested/deeper/frame.png");
        let image = app.capture(&out, 240).unwrap();
        assert!(matches!(image.bytes(), [_, ..]));
        assert!(out.exists());
    }

    #[test]
    fn test_usage_settled_returns_a_frame_once_two_readbacks_match() {
        let mut app = PreviewApp::from_scene(
            &scene_with("demo", &["one room", "no rooms!"], None),
            &preview::Config::default(),
        );
        let dir = tempfile::tempdir().unwrap();
        let out = dir.path().join("settled.png");
        let image = app.settled(&out, 240, 1).unwrap();
        assert!(matches!(image.bytes(), [_, ..]));
        assert!(out.exists());
    }

    #[test]
    fn test_usage_a_zero_frame_budget_reports_no_frame() {
        let mut app = PreviewApp::from_scene(
            &scene_with("demo", &["one room", "no rooms!"], None),
            &preview::Config::default(),
        );
        let dir = tempfile::tempdir().unwrap();
        let out = dir.path().join("missing.png");
        assert!(matches!(
            app.capture(&out, 0),
            Err(crate::Error::NoFrame(0))
        ));
    }

    #[test]
    fn test_usage_render_states_writes_one_png_per_state_named_after_the_scene() {
        let dir = tempfile::tempdir().unwrap();
        let staging = deliver::Artifact::staging_root(dir.path(), "demo");
        let rendered = scene_with("demo", &["one room", "no rooms!"], None)
            .render_states(&fast_config(dir.path()), &staging)
            .unwrap();
        assert_eq!(rendered.len(), 2);
        for state in &rendered {
            assert!(state.path.exists(), "{} missing", state.path.display());
            assert!(state.label.len() > 1);
            assert_eq!(state.pixel_hash.len(), crate::deliver::HASH_CHARS);
        }
        assert!(
            rendered[0]
                .path
                .to_string_lossy()
                .contains("demo__one_room")
        );
    }

    #[test]
    fn test_usage_render_states_rejects_an_invalid_scene_before_rendering() {
        let dir = tempfile::tempdir().unwrap();
        let broken = scene::Scene {
            states: Vec::new(),
            ..scene_with("demo", &["one room", "no rooms!"], None)
        };
        let staging = deliver::Artifact::staging_root(dir.path(), "demo");
        assert!(
            broken
                .render_states(&fast_config(dir.path()), &staging)
                .is_err()
        );
    }

    #[test]
    fn test_usage_from_scene_warmup_and_target_are_usable_together() {
        let mut app = PreviewApp::from_scene(
            &scene_with("demo", &["one room", "no rooms!"], None),
            &preview::Config::default(),
        );
        app.warmup();
        app.app.update();
        assert_ne!(app.target.clone(), Handle::default());
    }
    // no test_usage necessary
}
