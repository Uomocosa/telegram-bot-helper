use bevy::asset::Assets;
use bevy::camera::RenderTarget;
use bevy::image::Image;
use bevy::prelude::*;
use bevy::ui::IsDefaultUiCamera;
use bevy::window::PrimaryWindow;

use crate::PreviewApp;
use crate::preview;

pub fn spawn_offscreen_ui_camera(app: &mut App, config: &preview::Config) -> Handle<Image> {
    let target = app
        .world_mut()
        .resource_mut::<Assets<Image>>()
        .add(PreviewApp::offscreen_image(config.width, config.height));
    app.world_mut().spawn((
        Camera2d,
        IsDefaultUiCamera,
        RenderTarget::Image(target.clone().into()),
    ));
    app.world_mut().spawn((
        PrimaryWindow,
        Window {
            resolution: (config.width, config.height).into(),
            ..default()
        },
    ));
    target
}

#[cfg(test)]
mod tests {
    use bevy::prelude::{App, Entity, Handle, With};

    use crate::PreviewApp;
    use crate::testing::fast_config;

    use super::spawn_offscreen_ui_camera;

    #[test]
    fn test_usage() {
        let dir = tempfile::tempdir().unwrap();
        let config = fast_config(dir.path());
        let mut app = App::new();
        PreviewApp::add_headless_plugins(&mut app);
        let target = spawn_offscreen_ui_camera(&mut app, &config);
        assert_ne!(target, Handle::default());
        let world = app.world_mut();
        let mut cameras = world.query_filtered::<Entity, With<bevy::camera::RenderTarget>>();
        assert_eq!(cameras.iter(world).count(), 1);
    }
}
