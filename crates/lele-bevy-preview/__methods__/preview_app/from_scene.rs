use bevy::prelude::App;

use crate::PreviewApp;
use crate::preview;
use crate::scene;

#[must_use]
pub fn from_scene(scene: &scene::Scene, config: &preview::Config) -> PreviewApp {
    let mut app = App::new();
    app.init_resource::<scene::Clock>();
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
    use bevy::prelude::Handle;

    use crate::testing::fast_config;
    use crate::testing::scene_with;

    use super::from_scene;

    #[test]
    fn test_usage() {
        let dir = tempfile::tempdir().unwrap();
        let config = fast_config(dir.path());
        let app = from_scene(&scene_with("demo", &["one room"], None), &config);
        assert_ne!(app.target, Handle::default());
        assert_eq!(app.config.out_dir, dir.path());
    }
}
