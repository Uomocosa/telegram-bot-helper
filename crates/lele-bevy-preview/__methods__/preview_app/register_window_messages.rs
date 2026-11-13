use bevy::prelude::App;

pub fn register_window_messages(app: &mut App) {
    app.add_message::<bevy::window::WindowEvent>()
        .add_message::<bevy::window::WindowResized>()
        .add_message::<bevy::window::WindowCreated>()
        .add_message::<bevy::window::WindowClosing>()
        .add_message::<bevy::window::WindowClosed>()
        .add_message::<bevy::window::WindowCloseRequested>()
        .add_message::<bevy::window::WindowDestroyed>()
        .add_message::<bevy::window::WindowFocused>()
        .add_message::<bevy::window::WindowOccluded>()
        .add_message::<bevy::window::WindowScaleFactorChanged>()
        .add_message::<bevy::window::WindowBackendScaleFactorChanged>()
        .add_message::<bevy::window::WindowMoved>()
        .add_message::<bevy::window::WindowThemeChanged>()
        .add_message::<bevy::window::Ime>();
}

#[cfg(test)]
mod tests {
    use super::register_window_messages;
    use bevy::prelude::App;

    #[test]
    fn test_usage() {
        let mut app = App::new();
        register_window_messages(&mut app);
        let world = app.world();
        assert!(
            world.contains_resource::<bevy::ecs::message::Messages<bevy::window::WindowResized>>()
        );
        assert!(world.contains_resource::<bevy::ecs::message::Messages<bevy::window::Ime>>());
    }
}
