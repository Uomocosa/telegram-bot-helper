use bevy::prelude::App;

use crate::PreviewApp;
use crate::preview;
use crate::scene;

#[must_use]
pub fn from_scene(scene: &scene::Scene, config: &preview::Config) -> PreviewApp {
    let mut app = App::new();
    (scene.build)(&mut app);
    let target = PreviewApp::install_harness(&mut app, config);
    PreviewApp {
        app,
        target,
        config: config.clone(),
    }
}

#[cfg(test)]
mod tests {
    use super::from_scene;

    #[test]
    fn test_usage() {
        let _ = from_scene;
    }
}
