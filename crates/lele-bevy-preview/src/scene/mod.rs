#[path = "__basic__/mod.rs"]
mod basic;
pub use basic::components::PreviewCenter;
pub use basic::constants::{MAX_FPS, MAX_STATES, MAX_TIMELINE_FRAMES};
pub use basic::structs::{State, Timeline};
mod kind;
pub use kind::Kind;
mod scene;
pub use scene::Scene;
#[path = "__bevy_systems__/mod.rs"]
pub mod bevy_systems;
