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
    use super::spawn_offscreen_ui_camera;

    #[test]
    fn test_usage() {
        let _ = spawn_offscreen_ui_camera;
    }
}
