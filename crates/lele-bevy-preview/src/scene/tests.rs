#[cfg(test)]
mod tests {
    use crate::scene;
    use bevy::ecs::world::World;
    use bevy::prelude::App;

    use crate::Error;
    use crate::scene::Scene;
    use crate::scene::basic::structs::{State, Timeline};

    fn noop(_world: &mut World) {}
    fn noop_build(_app: &mut App) {}

    fn scene() -> scene::Scene {
        scene::Scene {
            name: String::from("spawn_root"),
            kind: scene::Kind::System,
            build: noop_build,
            states: vec![State {
                label: String::from("empty"),
                apply: noop,
            }],
            timeline: Some(Timeline {
                label: String::from("press"),
                frames: 4,
                fps: 60,
                apply: noop,
            }),
        }
    }

    fn spec(frames: u32, fps: u32) -> Timeline {
        Timeline {
            label: String::from("press"),
            frames,
            fps,
            apply: noop,
        }
    }

    #[test]
    fn test_usage() {
        assert!(scene().check().is_ok());
        assert!(matches!(scene().states.first(), Some(State { .. })));
        assert!(matches!(scene().timeline, Some(Timeline { frames: 4, .. })));
    }

    #[test]
    fn test_usage_rejects_empty_states_zero_frames_and_blank_names() {
        let no_states = scene::Scene {
            states: Vec::new(),
            ..scene()
        };
        assert!(matches!(no_states.check(), Err(Error::Scene(_))));
        let zero_frames = scene::Scene {
            timeline: Some(spec(0, 60)),
            ..scene()
        };
        assert!(zero_frames.check().is_err());
        let unnamed = scene::Scene {
            name: String::new(),
            kind: scene::Kind::System,
            ..scene()
        };
        assert!(unnamed.check().is_err());
    }

    #[test]
    fn test_usage_rejects_bad_labels_caps_and_fps() {
        let blank_label = scene::Scene {
            states: vec![State {
                label: String::from("  "),
                apply: noop,
            }],
            ..scene()
        };
        assert!(blank_label.check().is_err());
        assert!(
            Scene {
                timeline: Some(spec(2, 0)),
                ..scene()
            }
            .check()
            .is_err()
        );
        assert!(
            Scene {
                timeline: Some(spec(2, 5000)),
                ..scene()
            }
            .check()
            .is_err()
        );
        assert!(
            Scene {
                timeline: Some(spec(5000, 60)),
                ..scene()
            }
            .check()
            .is_err()
        );
        let too_many = scene::Scene {
            states: (0..40)
                .map(|_| State {
                    label: String::from("s"),
                    apply: noop,
                })
                .collect(),
            ..scene()
        };
        assert!(too_many.check().is_err());
    }

    #[test]
    fn test_usage_centered_root_fills_the_window_and_centres() {
        let root = Scene::centered_root();
        assert_eq!(root.width, bevy::ui::Val::Percent(100.0));
        assert_eq!(root.height, bevy::ui::Val::Percent(100.0));
        assert_eq!(root.justify_content, bevy::ui::JustifyContent::Center);
        assert_eq!(root.align_items, bevy::ui::AlignItems::Center);
    }

    // no test_usage necessary
}
