use bevy::picking::backend::HitData;
use bevy::prelude::Entity;

pub const MAX_STATES: usize = 32;
pub const MAX_TIMELINE_FRAMES: u32 = 600;
pub const MAX_FPS: u32 = 120;
pub const SYNTHETIC_HIT: HitData = HitData {
    camera: Entity::PLACEHOLDER,
    depth: 0.0,
    position: None,
    normal: None,
    extra: None,
};
