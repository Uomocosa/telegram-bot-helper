use bevy::prelude::*;

#[must_use]
pub fn centered_root() -> Node {
    Node {
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        flex_direction: FlexDirection::Column,
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        row_gap: Val::Px(6.0),
        ..default()
    }
}

#[cfg(test)]
mod tests {
    use bevy::prelude::*;

    use super::centered_root;

    #[derive(Component)]
    struct Column;

    #[test]
    fn test_usage() {
        let root = centered_root();
        assert_eq!(root.width, Val::Percent(100.0));
        assert_eq!(root.justify_content, JustifyContent::Center);
        assert_eq!(root.align_items, AlignItems::Center);
    }

    fn row(label: &str) -> impl Bundle {
        (
            Node {
                padding: UiRect::axes(Val::Px(16.0), Val::Px(8.0)),
                ..default()
            },
            BackgroundColor(Color::srgb(0.20, 0.24, 0.32)),
            children![Text::new(label.to_string())],
        )
    }

    fn spawn_column(mut commands: Commands) {
        let root = commands
            .spawn((Column, lele_bevy_preview::scene::centered_root()))
            .id();
        commands.spawn((ChildOf(root), row("first")));
    }

    fn preview_build(app: &mut App) {
        app.add_systems(Startup, spawn_column);
    }

    const fn preview_noop(_world: &mut World) {}

    fn add_two_rows(world: &mut World) {
        let mut columns = world.query_filtered::<Entity, With<Column>>();
        let Ok(root) = columns.single(world) else {
            return;
        };
        world.spawn((ChildOf(root), row("second")));
        world.spawn((ChildOf(root), row("third")));
    }

    #[test]
    fn centered_root_ui_png_preview() {
        let out = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("ui_preview");
        let report = lele_bevy_preview::run(
            &lele_bevy_preview::scene::Scene {
                name: String::from("centered_root"),
                kind: lele_bevy_preview::scene::Kind::Component,
                build: preview_build,
                states: vec![
                    lele_bevy_preview::scene::State {
                        label: String::from("one row"),
                        apply: preview_noop,
                    },
                    lele_bevy_preview::scene::State {
                        label: String::from("three rows"),
                        apply: add_two_rows,
                    },
                ],
                timeline: None,
            },
            &lele_bevy_preview::preview::Config {
                out_dir: out,
                ..Default::default()
            },
            env!("CARGO_PKG_NAME"),
        )
        .expect("preview");
        lele_bevy_preview::layout::assert_centered(&report, 2).expect("centred");
    }
}
