use bevy::prelude::*;

use crate::scene;

pub fn center_roots(
    mut commands: Commands,
    mut roots: Query<(Entity, &mut Node), Without<ChildOf>>,
) {
    let center = commands.spawn(scene::centered_root()).id();
    for (entity, mut node) in &mut roots {
        node.position_type = PositionType::Relative;
        node.top = Val::Auto;
        node.left = Val::Auto;
        node.right = Val::Auto;
        node.bottom = Val::Auto;
        commands.entity(entity).insert(ChildOf(center));
    }
}
// no test_usage necessary
