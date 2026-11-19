use std::time::Duration;

use bevy::image::Image;
use bevy::prelude::*;
use bevy::time::TimeUpdateStrategy;

use crate::PreviewApp;
use crate::preview;
use crate::preview::{CLEAR_B, CLEAR_G, CLEAR_R};

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
    use bevy::prelude::{App, ClearColor};

    use crate::testing::fast_config;

    use super::install_harness;

    #[test]
    fn test_usage() {
        let dir = tempfile::tempdir().unwrap();
        let config = fast_config(dir.path());
        let mut app = App::new();
        let target = install_harness(&mut app, &config);
        assert_ne!(target, bevy::prelude::Handle::default());
        assert!(
            app.world()
                .contains_resource::<bevy::time::TimeUpdateStrategy>()
        );
        assert!(app.world().contains_resource::<ClearColor>());
    }
}
