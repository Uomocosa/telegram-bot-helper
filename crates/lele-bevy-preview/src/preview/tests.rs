#[cfg(test)]
mod tests {
    use crate::PreviewApp;
    use crate::preview;
    use bevy::prelude::*;

    use crate::scene;

    fn noop(_world: &mut World) {}
    fn noop_build(_app: &mut App) {}

    fn scene() -> scene::Scene {
        scene::Scene {
            name: String::from("demo"),
            kind: scene::Kind::System,
            build: noop_build,
            states: vec![
                crate::scene::basic::structs::State {
                    label: String::from("one room"),
                    apply: noop,
                },
                crate::scene::basic::structs::State {
                    label: String::from("no rooms!"),
                    apply: noop,
                },
            ],
            timeline: None,
        }
    }

    fn fast(dir: &std::path::Path) -> preview::Config {
        preview::Config {
            out_dir: dir.to_path_buf(),
            warmup_frames: 5,
            max_capture_frames: 240,
            min_distinct_colors: 1,
            ..preview::Config::default()
        }
    }

    #[test]
    fn test_usage() {
        let config = preview::Config::default();
        assert_eq!(config.width, preview::basic::constants::DEFAULT_WIDTH);
        assert_eq!(config.height, preview::basic::constants::DEFAULT_HEIGHT);
        assert_eq!(
            config.warmup_frames,
            preview::basic::constants::DEFAULT_WARMUP_FRAMES
        );
        assert!(config.out_dir.ends_with("ui_preview"));
    }

    #[test]
    fn test_usage_from_scene_produces_an_offscreen_target_and_a_camera() {
        let mut app = PreviewApp::from_scene(&scene(), &preview::Config::default());
        assert_ne!(app.target.clone(), Handle::default());
        let world = app.app.world_mut();
        let mut cameras = world.query_filtered::<Entity, With<bevy::camera::RenderTarget>>();
        assert!(cameras.iter(world).count() > 0);
    }

    #[test]
    fn test_usage_capture_writes_a_non_empty_png_and_creates_parents() {
        let mut app = PreviewApp::from_scene(&scene(), &preview::Config::default());
        let dir = tempfile::tempdir().unwrap();
        let out = dir.path().join("nested/deeper/frame.png");
        let image = app.capture(&out, 240).unwrap();
        assert!(matches!(image.bytes(), [_, ..]));
        assert!(out.exists());
    }

    #[test]
    fn test_usage_settled_returns_a_frame_once_two_readbacks_match() {
        let mut app = PreviewApp::from_scene(&scene(), &preview::Config::default());
        let dir = tempfile::tempdir().unwrap();
        let out = dir.path().join("settled.png");
        let image = app.settled(&out, 240, 1).unwrap();
        assert!(matches!(image.bytes(), [_, ..]));
        assert!(out.exists());
    }

    #[test]
    fn test_usage_a_zero_frame_budget_reports_no_frame() {
        let mut app = PreviewApp::from_scene(&scene(), &preview::Config::default());
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
        let rendered = scene().render_states(&fast(dir.path())).unwrap();
        assert_eq!(rendered.len(), 2);
        for state in &rendered {
            assert!(state.path.exists(), "{} missing", state.path.display());
            assert!(state.label.len() > 1);
            assert_eq!(
                state.pixel_hash.len(),
                crate::deliver::basic::constants::HASH_CHARS
            );
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
            ..scene()
        };
        assert!(broken.render_states(&fast(dir.path())).is_err());
    }

    #[test]
    fn test_usage_from_scene_warmup_and_target_are_usable_together() {
        let mut app = PreviewApp::from_scene(&scene(), &preview::Config::default());
        app.warmup();
        app.app.update();
        assert_ne!(app.target.clone(), Handle::default());
    }
    // no test_usage necessary
}
