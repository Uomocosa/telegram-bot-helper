use std::path::Path;

use crate::deliver;
use crate::preview;
use crate::scene;

pub fn drop_positions_staged(
    scene: &scene::Scene,
    fresh: &mut deliver::FreshDelivery,
    positions: &[(usize, bool)],
    config: &preview::Config,
    staging_root: &Path,
) {
    for (position, is_clip) in positions {
        if *is_clip {
            if let Some(clip) = fresh.clip.as_mut() {
                clip.drop_staged_file(staging_root, &scene.name);
                let delivered = clip.artifact.delivered_path(&config.out_dir);
                clip.path.clone_from(&delivered);
                clip.clear_unsent_entry(&config.out_dir, &scene.name);
            }
        } else if let Some(capture) = fresh.captures.get_mut(*position) {
            capture.drop_staged_file(staging_root, &scene.name);
            let delivered = capture.artifact.delivered_path(&config.out_dir);
            capture.path.clone_from(&delivered);
            capture.clear_unsent_entry(&config.out_dir, &scene.name);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::drop_positions_staged;
    use crate::deliver;
    use crate::preview;
    use crate::scene;
    use std::path::Path;

    #[test]
    fn test_usage() {
        let owner = scene::Scene {
            name: String::from("ui"),
            kind: scene::Kind::System,
            build: |_app| {},
            states: Vec::new(),
            timeline: None,
        };
        let config = preview::Config::default();
        let mut fresh = deliver::FreshDelivery {
            captures: Vec::new(),
            clip: None,
        };
        drop_positions_staged(
            &owner,
            &mut fresh,
            &[(0, false), (0, true)],
            &config,
            Path::new("staging"),
        );
        assert_eq!(fresh.captures.len(), 0);
        assert!(fresh.clip.is_none());
    }
}
