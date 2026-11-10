#[path = "__basic__/mod.rs"]
mod basic;
pub use basic::constants::{
    BACKGROUND_A, BACKGROUND_B, BACKGROUND_G, BACKGROUND_R, CLEAR_B, CLEAR_G, CLEAR_R,
    DEFAULT_HEIGHT, DEFAULT_LEAD_IN_FRAMES, DEFAULT_MAX_CAPTURE_FRAMES,
    DEFAULT_MIN_DISTINCT_COLORS, DEFAULT_WARMUP_FRAMES, DEFAULT_WIDTH, SETTLE_CAPTURES,
};
pub use basic::structs::RenderedState;

pub mod captured_image;
pub mod config;

pub use captured_image::CapturedImage;
pub use config::Config;
