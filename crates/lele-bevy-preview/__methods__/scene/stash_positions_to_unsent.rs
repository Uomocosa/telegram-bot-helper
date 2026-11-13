use std::path::Path;

use crate::deliver;
use crate::preview;
use crate::scene;

pub fn stash_positions_to_unsent(
    scene: &scene::Scene,
    fresh: &mut deliver::FreshDelivery,
    positions: &[(usize, bool)],
    config: &preview::Config,
    staging_root: &Path,
) {
    for (position, is_clip) in positions {
        if *is_clip {
            if let Some(clip) = fresh.clip.as_mut() {
                let _ = clip.stash_staged_to_unsent(&config.out_dir, &scene.name, staging_root);
            }
        } else if let Some(capture) = fresh.captures.get_mut(*position) {
            let _ = capture.stash_staged_to_unsent(&config.out_dir, &scene.name, staging_root);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::stash_positions_to_unsent;
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
        stash_positions_to_unsent(
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
