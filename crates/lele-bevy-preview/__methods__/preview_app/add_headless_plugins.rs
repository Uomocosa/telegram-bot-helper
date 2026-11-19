use bevy::prelude::*;

use crate::PreviewApp;

pub fn add_headless_plugins(app: &mut App) {
    app.add_plugins(
        DefaultPlugins
            .build()
            .disable::<bevy::winit::WinitPlugin>()
            .disable::<bevy::window::WindowPlugin>()
            .disable::<bevy::a11y::AccessibilityPlugin>()
            .disable::<bevy::input_focus::InputFocusPlugin>()
            .disable::<bevy::input_focus::InputDispatchPlugin>(),
    );
    PreviewApp::register_window_messages(app);
}

#[cfg(test)]
mod tests {
    use bevy::prelude::App;

    use super::add_headless_plugins;

    #[test]
    fn test_usage() {
        let mut app = App::new();
        add_headless_plugins(&mut app);
        assert!(app.is_plugin_added::<bevy::time::TimePlugin>());
        assert!(!app.is_plugin_added::<bevy::winit::WinitPlugin>());
    }
}
