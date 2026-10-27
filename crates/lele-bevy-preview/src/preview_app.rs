use std::path::Path;
use std::time::Duration;

use atomic_delegate_macros::atomic_delegates;
use bevy::image::Image;
use bevy::prelude::*;

use crate::Error;
use crate::basic;
use crate::preview;
use crate::scene;

pub struct PreviewApp {
    pub app: App,
    pub target: Handle<Image>,
    pub config: preview::Config,
}

#[atomic_delegates]
impl PreviewApp {
    #[must_use]
    pub fn from_scene(scene: &scene::Scene, config: &preview::Config) -> Self {}
    pub fn advance(&mut self, step: Duration) {}
    pub fn capture(&mut self, out: &Path, budget: u32) -> Result<preview::CapturedImage, Error> {}
    pub fn settled(
        &mut self,
        out: &Path,
        budget: u32,
        min_distinct_colors: usize,
    ) -> Result<preview::CapturedImage, Error> {
    }
}

#[rustfmt::skip]
impl PreviewApp {
    pub fn warmup(&mut self) { for _ in 0..self.config.warmup_frames { self.app.update(); } }
}

#[atomic_delegates]
impl PreviewApp {
    pub(crate) fn request_screenshot(
        &mut self,
        target: Handle<Image>,
        out: &Path,
        budget: u32,
    ) -> Option<Image> {
    }
    pub(crate) fn staging_path_for(out: &Path) -> Result<std::path::PathBuf, Error> {}
    pub(crate) fn install_harness(app: &mut App, config: &preview::Config) -> Handle<Image> {}
    pub(crate) fn add_headless_plugins(app: &mut App) {}
    pub(crate) fn spawn_offscreen_ui_camera(
        app: &mut App,
        config: &preview::Config,
    ) -> Handle<Image> {
    }
    pub(crate) fn offscreen_image(width: u32, height: u32) -> Image {}
}

#[rustfmt::skip]
impl PreviewApp {
    pub(crate) fn drain_in_flight_screenshot(&mut self) { for _ in 0..basic::constants::PUMP_FRAMES { self.app.update(); } }
    pub(crate) fn register_window_messages(app: &mut App) { app.add_message::<bevy::window::WindowEvent>() .add_message::<bevy::window::WindowResized>() .add_message::<bevy::window::WindowCreated>() .add_message::<bevy::window::WindowClosing>() .add_message::<bevy::window::WindowClosed>() .add_message::<bevy::window::WindowCloseRequested>() .add_message::<bevy::window::WindowDestroyed>() .add_message::<bevy::window::WindowFocused>() .add_message::<bevy::window::WindowOccluded>() .add_message::<bevy::window::WindowScaleFactorChanged>() .add_message::<bevy::window::WindowBackendScaleFactorChanged>() .add_message::<bevy::window::WindowMoved>() .add_message::<bevy::window::WindowThemeChanged>() .add_message::<bevy::window::Ime>(); }
}

#[cfg(test)]
mod tests {
    use super::PreviewApp;

    #[test]
    fn test_usage() {
        let _ = PreviewApp::from_scene;
        let mut app = bevy::prelude::App::new();
        PreviewApp::register_window_messages(&mut app);
        let world = app.world();
        assert!(
            world.contains_resource::<bevy::ecs::message::Messages<bevy::window::WindowResized>>()
        );
        assert!(world.contains_resource::<bevy::ecs::message::Messages<bevy::window::Ime>>());
    }
}
