use bevy::picking::hover::PointerCaptureMap;
use bevy::picking::pointer::PointerId;
use bevy::prelude::{Component, Entity, With, World};
use tracing::warn;

use crate::scene;

pub fn hover<C: Component>(world: &mut World) {
    let target = world.query_filtered::<Entity, With<C>>().iter(world).next();
    let Some(target) = target else {
        warn!(
            "no entity with component {} to hover",
            std::any::type_name::<C>()
        );
        return;
    };
    world.resource_mut::<PointerCaptureMap>().capture(
        PointerId::Mouse,
        target,
        scene::SYNTHETIC_HIT,
    );
}

#[cfg(test)]
mod tests {
    use bevy::picking::hover::PointerCaptureMap;
    use bevy::picking::pointer::PointerId;
    use bevy::prelude::World;

    use super::hover;
    use crate::scene::PreviewCenter;

    #[test]
    fn test_usage() {
        let mut world = World::new();
        world.init_resource::<PointerCaptureMap>();
        let target = world.spawn(PreviewCenter).id();
        hover::<PreviewCenter>(&mut world);
        let captures = world.resource::<PointerCaptureMap>();
        assert!(captures.is_captured(&PointerId::Mouse));
        assert_eq!(
            captures.get(&PointerId::Mouse).map(|(entity, _hit)| entity),
            Some(target)
        );
    }
}
