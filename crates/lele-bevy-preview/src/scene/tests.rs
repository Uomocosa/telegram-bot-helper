use crate::scene;
use bevy::ecs::world::World;
use bevy::prelude::App;

use crate::Error;
use crate::scene::Scene;
use crate::scene::basic::structs::{State, Timeline};

// needed helper: fixture shared by the tests in this file
fn noop(_world: &mut World) {}
// needed helper: fixture shared by the tests in this file
fn noop_build(_app: &mut App) {}

// needed helper: fixture shared by the tests in this file
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

// needed helper: fixture shared by the tests in this file
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
    assert!(scene::check::check(&scene()).is_ok());
    assert!(matches!(scene().states.first(), Some(State { .. })));
    assert!(matches!(scene().timeline, Some(Timeline { frames: 4, .. })));
}

#[test]
fn test_usage_rejects_empty_states_zero_frames_and_blank_names() {
    let no_states = scene::Scene {
        states: Vec::new(),
        ..scene()
    };
    assert!(matches!(
        scene::check::check(&no_states),
        Err(Error::Scene(_))
    ));
    let zero_frames = scene::Scene {
        timeline: Some(spec(0, 60)),
        ..scene()
    };
    assert!(scene::check::check(&zero_frames).is_err());
    let unnamed = scene::Scene {
        name: String::new(),
        kind: scene::Kind::System,
        ..scene()
    };
    assert!(scene::check::check(&unnamed).is_err());
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
    assert!(scene::check::check(&blank_label).is_err());
    assert!(
        scene::check::check(&Scene {
            timeline: Some(spec(2, 0)),
            ..scene()
        })
        .is_err()
    );
    assert!(
        scene::check::check(&Scene {
            timeline: Some(spec(2, 5000)),
            ..scene()
        })
        .is_err()
    );
    assert!(
        scene::check::check(&Scene {
            timeline: Some(spec(5000, 60)),
            ..scene()
        })
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
    assert!(scene::check::check(&too_many).is_err());
}

#[test]
fn test_usage_centered_root_fills_the_window_and_centres() {
    let root = scene::centered_root();
    assert_eq!(root.width, bevy::ui::Val::Percent(100.0));
    assert_eq!(root.height, bevy::ui::Val::Percent(100.0));
    assert_eq!(root.justify_content, bevy::ui::JustifyContent::Center);
    assert_eq!(root.align_items, bevy::ui::AlignItems::Center);
}

// needed helper: fixture shared by the tests in this file
fn spawn_corner_node(mut commands: bevy::ecs::system::Commands) {
    commands.spawn(bevy::ui::Node {
        position_type: bevy::ui::PositionType::Absolute,
        top: bevy::ui::Val::Px(48.0),
        left: bevy::ui::Val::Px(12.0),
        ..Default::default()
    });
}

#[test]
fn test_usage_center_roots_parents_every_root_under_one_centred_root() {
    use bevy::ecs::schedule::IntoScheduleConfigs;
    let mut app = App::new();
    app.add_systems(
        bevy::app::Startup,
        (spawn_corner_node, scene::center_roots).chain(),
    );
    app.update();
    let world = app.world_mut();
    let mut roots = world.query_filtered::<&bevy::ui::Node, bevy::ecs::query::Without<bevy::ecs::hierarchy::ChildOf>>();
    let parentless: Vec<&bevy::ui::Node> = roots.iter(world).collect();
    assert_eq!(parentless.len(), 1);
    assert!(
        parentless
            .iter()
            .all(|node| node.justify_content == bevy::ui::JustifyContent::Center)
    );
    let mut moved = world
        .query_filtered::<&bevy::ui::Node, bevy::ecs::query::With<bevy::ecs::hierarchy::ChildOf>>();
    assert!(
        moved
            .iter(world)
            .all(|node| node.position_type == bevy::ui::PositionType::Relative)
    );
}
// no test_usage necessary
