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
    use crate::testing::scene_with;

    #[test]
    fn test_usage() {
        let config = preview::Config {
            out_dir: PathBuf::from("out"),
            ..preview::Config::default()
        };
        assert_eq!(
            scratch_dir(&scene_with("ui", &[], None), &config),
            PathBuf::from("out/ui_frames")
        );
        assert_eq!(
            scratch_dir(&scene_with("press_flow", &[], None), &config),
            PathBuf::from("out/press_flow_frames")
        );
    }
}
