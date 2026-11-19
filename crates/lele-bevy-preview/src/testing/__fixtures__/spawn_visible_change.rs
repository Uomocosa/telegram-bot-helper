use bevy::ecs::world::World;

pub fn spawn_visible_change(world: &mut World) {
    world.spawn((
        bevy::ui::Node {
            width: bevy::ui::Val::Px(200.0),
            height: bevy::ui::Val::Px(120.0),
            ..Default::default()
        },
        bevy::ui::BackgroundColor(bevy::color::Color::srgb(0.9, 0.2, 0.2)),
    ));
}

#[cfg(test)]
mod tests {
    use super::spawn_visible_change;
    use bevy::ecs::world::World;

    #[test]
    fn test_usage() {
        let mut world = World::new();
        let before = world.iter_entities().count();
        spawn_visible_change(&mut world);
        assert_eq!(world.iter_entities().count(), before + 1);
    }
}
