use bevy::ecs::world::World;

pub struct State {
    pub label: String,
    pub apply: fn(&mut World),
}

pub struct Timeline {
    pub label: String,
    pub frames: u32,
    pub fps: u32,
    pub apply: fn(&mut World),
}
