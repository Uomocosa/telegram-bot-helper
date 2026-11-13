use std::path::PathBuf;

use crate::preview;
use crate::scene;

#[must_use]
pub fn scratch_dir(scene: &scene::Scene, config: &preview::Config) -> PathBuf {
    let folder = format!("{}_frames", scene.name);
    config.out_dir.join(folder)
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::scratch_dir;
    use crate::preview;
    use crate::scene;

    fn scene_named(name: &str) -> scene::Scene {
        scene::Scene {
            name: String::from(name),
            kind: scene::Kind::System,
            build: |_app| {},
            states: Vec::new(),
            timeline: None,
        }
    }

    #[test]
    fn test_usage() {
        let config = preview::Config {
            out_dir: PathBuf::from("out"),
            ..preview::Config::default()
        };
        assert_eq!(
            scratch_dir(&scene_named("ui"), &config),
            PathBuf::from("out/ui_frames")
        );
        assert_eq!(
            scratch_dir(&scene_named("press_flow"), &config),
            PathBuf::from("out/press_flow_frames")
        );
    }
}
