use bevy::prelude::{Res, Resource, Time, World};
use derive_more::Deref;

#[derive(Resource, Debug, Default, Clone, Copy, PartialEq, Deref)]
pub struct Clock(Option<f32>);

#[rustfmt::skip]
impl Clock {
    pub fn start(world: &mut World) { world.insert_resource(Self(Some(world.resource::<Time>().elapsed_secs()))); }
    #[must_use]
    pub fn is_started(clock: Res<Self>) -> bool { clock.into_inner().is_some() }
}

impl Clock {
    #[must_use]
    pub fn elapsed(&self, time: &Time) -> Option<f32> {
        let Self(start) = self;
        start.map(|start| time.elapsed_secs() - start)
    }
}

#[cfg(test)]
mod tests {
    use bevy::ecs::system::RunSystemOnce;
    use bevy::prelude::{Time, World};

    use super::Clock;

    #[test]
    fn test_usage() {
        let mut world = World::new();
        world.init_resource::<Time>();
        world.init_resource::<Clock>();
        let clock = Clock::default();
        assert_eq!(clock.elapsed(world.resource::<Time>()), None);
        assert!(!world.run_system_once(Clock::is_started).unwrap());
        Clock::start(&mut world);
        assert!(
            world
                .resource::<Clock>()
                .elapsed(world.resource::<Time>())
                .is_some()
        );
        assert!(world.run_system_once(Clock::is_started).unwrap());
    }
}
