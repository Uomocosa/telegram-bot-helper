use std::time::Duration;

use bevy::image::Image;
use bevy::prelude::*;
use bevy::time::TimeUpdateStrategy;

use crate::PreviewApp;
use crate::preview;
use crate::preview::basic::constants::{CLEAR_B, CLEAR_G, CLEAR_R};

pub fn install_harness(app: &mut App, config: &preview::Config) -> Handle<Image> {
    PreviewApp::add_headless_plugins(app);
    app.insert_resource(TimeUpdateStrategy::ManualDuration(Duration::ZERO));
    app.insert_resource(ClearColor(Color::srgb(CLEAR_R, CLEAR_G, CLEAR_B)));
    app.init_resource::<bevy::input_focus::InputFocus>();
    app.init_resource::<bevy::input_focus::InputFocusVisible>();
    let target = PreviewApp::spawn_offscreen_ui_camera(app, config);
    app.finish();
    app.cleanup();
    target
}

#[cfg(test)]
mod tests {
    use super::install_harness;

    #[test]
    fn test_usage() {
        let _ = install_harness;
    }
}
