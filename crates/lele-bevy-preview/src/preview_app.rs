use std::path::Path;
use std::time::Duration;

use atomic_delegate_macros::atomic_delegates;
use bevy::image::Image;
use bevy::prelude::*;

use crate::Error;
use crate::preview;
use crate::scene;

pub struct PreviewApp {
    pub app: App,
    pub target: Handle<Image>,
    pub config: preview::Config,
}

#[rustfmt::skip]
impl PreviewApp {
    pub fn warmup(&mut self) { for _ in 0..self.config.warmup_frames { self.app.update(); } }
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

#[cfg(test)]
mod tests {
    use super::PreviewApp;

    #[test]
    fn test_usage() {
        let _ = PreviewApp::from_scene;
    }
}
