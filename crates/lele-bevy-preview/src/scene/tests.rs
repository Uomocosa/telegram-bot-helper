#[cfg(test)]
mod tests {
    use crate::Error;
    use crate::scene;
    use crate::scene::Scene;
    use crate::scene::{State, Timeline};
    use crate::testing::noop;
    use crate::testing::scene_with;
    use crate::testing::timeline_with;

    #[test]
    fn test_usage() {
        assert!(
            scene_with(
                "spawn_root",
                &["empty"],
                Some(timeline_with("press", 4, 60, noop))
            )
            .check()
            .is_ok()
        );
        assert!(matches!(
            scene_with(
                "spawn_root",
                &["empty"],
                Some(timeline_with("press", 4, 60, noop))
            )
            .states
            .first(),
            Some(State { .. })
        ));
        assert!(matches!(
            scene_with(
                "spawn_root",
                &["empty"],
                Some(timeline_with("press", 4, 60, noop))
            )
            .timeline,
            Some(Timeline { frames: 4, .. })
        ));
    }

    #[test]
    fn test_usage_rejects_empty_states_zero_frames_and_blank_names() {
        let no_states = scene::Scene {
            states: Vec::new(),
            ..scene_with(
                "spawn_root",
                &["empty"],
                Some(timeline_with("press", 4, 60, noop)),
            )
        };
        assert!(matches!(no_states.check(), Err(Error::Scene(_))));
        let zero_frames = scene::Scene {
            timeline: Some(timeline_with("press", 0, 60, noop)),
            ..scene_with(
                "spawn_root",
                &["empty"],
                Some(timeline_with("press", 4, 60, noop)),
            )
        };
        assert!(zero_frames.check().is_err());
        let unnamed = scene::Scene {
            name: String::new(),
            kind: scene::Kind::System,
            ..scene_with(
                "spawn_root",
                &["empty"],
                Some(timeline_with("press", 4, 60, noop)),
            )
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
            ..scene_with(
                "spawn_root",
                &["empty"],
                Some(timeline_with("press", 4, 60, noop)),
            )
        };
        assert!(blank_label.check().is_err());
        assert!(
            Scene {
                timeline: Some(timeline_with("press", 2, 0, noop)),
                ..scene_with(
                    "spawn_root",
                    &["empty"],
                    Some(timeline_with("press", 4, 60, noop))
                )
            }
            .check()
            .is_err()
        );
        assert!(
            Scene {
                timeline: Some(timeline_with("press", 2, 5000, noop)),
                ..scene_with(
                    "spawn_root",
                    &["empty"],
                    Some(timeline_with("press", 4, 60, noop))
                )
            }
            .check()
            .is_err()
        );
        assert!(
            Scene {
                timeline: Some(timeline_with("press", 5000, 60, noop)),
                ..scene_with(
                    "spawn_root",
                    &["empty"],
                    Some(timeline_with("press", 4, 60, noop))
                )
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
            ..scene_with(
                "spawn_root",
                &["empty"],
                Some(timeline_with("press", 4, 60, noop)),
            )
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
