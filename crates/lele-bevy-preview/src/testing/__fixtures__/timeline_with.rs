use bevy::ecs::world::World;

use crate::scene;

pub fn timeline_with(label: &str, frames: u32, fps: u32, apply: fn(&mut World)) -> scene::Timeline {
    scene::Timeline {
        label: String::from(label),
        frames,
        fps,
        apply,
    }
}

#[cfg(test)]
mod tests {
    use super::timeline_with;
    use crate::testing::noop;

    #[test]
    fn test_usage() {
        assert_eq!(timeline_with("press", 4, 60, noop).frames, 4);
    }
}
