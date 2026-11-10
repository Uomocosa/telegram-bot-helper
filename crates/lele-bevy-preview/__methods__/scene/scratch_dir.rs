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
    use super::scratch_dir;

    #[test]
    fn test_usage() {
        let _ = scratch_dir;
    }
}
