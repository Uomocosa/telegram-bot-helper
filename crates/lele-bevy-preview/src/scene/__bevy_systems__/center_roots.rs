use bevy::prelude::*;

use crate::scene;

type Roots<'w, 's> =
    Query<'w, 's, (Entity, &'static mut Node), (Without<ChildOf>, Without<scene::PreviewCenter>)>;

pub fn center_roots(
    mut commands: Commands,
    centers: Query<Entity, With<scene::PreviewCenter>>,
    mut roots: Roots,
) {
    if roots.is_empty() {
        return;
    }
    let center = centers.single().unwrap_or_else(|_| {
        commands
            .spawn((scene::PreviewCenter, scene::Scene::centered_root()))
            .id()
    });
    for (entity, mut node) in &mut roots {
        node.position_type = PositionType::Relative;
        node.top = Val::Auto;
        node.left = Val::Auto;
        node.right = Val::Auto;
        node.bottom = Val::Auto;
        commands.entity(entity).insert(ChildOf(center));
    }
}

#[cfg(test)]
mod tests {
    use bevy::prelude::*;

    use super::center_roots;

    #[test]
    fn center_roots_ui_png_preview() {
        let out = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("ui_preview");
        let preview = lele_bevy_preview::scene::Scene {
            name: String::from("center_roots"),
            kind: lele_bevy_preview::scene::Kind::System,
            build: |app: &mut App| {
                app.add_systems(Startup, |mut commands: Commands| {
                    commands.spawn((
                        Node {
                            position_type: PositionType::Absolute,
                            top: Val::Px(24.0),
                            left: Val::Px(24.0),
                            padding: UiRect::axes(Val::Px(16.0), Val::Px(10.0)),
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.15, 0.35, 0.65)),
                        children![Text::new("pinned at 24/24")],
                    ));
                });
                app.add_systems(
                    Update,
                    center_roots.run_if(lele_bevy_preview::scene::Clock::is_started),
                );
            },
            states: vec![
                lele_bevy_preview::scene::State {
                    label: String::from("pinned in the corner"),
                    apply: |_world: &mut World| {},
                },
                lele_bevy_preview::scene::State {
                    label: String::from("centred"),
                    apply: lele_bevy_preview::scene::Clock::start,
                },
            ],
            timeline: None,
        };
        let report = preview
            .run(&lele_bevy_preview::preview::Config {
                out_dir: out,
                ..Default::default()
            })
            .expect("preview");
        let [before, after] = report.captures.as_slice() else {
            panic!("expected two captures");
        };
        assert!(before.assert_centered(2).is_err());
        after.assert_centered(2).expect("centred");
    }
}
