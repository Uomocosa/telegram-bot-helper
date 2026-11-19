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
    use std::path::Path;

    use super::stash_positions_to_unsent;
    use crate::deliver;
    use crate::preview;
    use crate::testing::capture;
    use crate::testing::scene_with;

    #[test]
    fn test_usage() {
        let dir = tempfile::tempdir().unwrap();
        let out = dir.path().join("out");
        let staging = dir.path().join("staging");
        std::fs::create_dir_all(&staging).unwrap();
        let changed_path = staging.join("ui__rest.png");
        let same_path = staging.join("ui__idle.png");
        std::fs::write(&changed_path, b"changed").unwrap();
        std::fs::write(&same_path, b"same").unwrap();
        let mut fresh = deliver::FreshDelivery {
            captures: vec![
                capture(
                    changed_path.clone(),
                    "rest",
                    deliver::Media::Png,
                    deliver::Status::Changed,
                ),
                capture(
                    same_path.clone(),
                    "idle",
                    deliver::Media::Png,
                    deliver::Status::Same,
                ),
            ],
            reasons: vec![
                deliver::Reason::ClustersExceedDrift,
                deliver::Reason::DriftWithinTolerance,
            ],
            diffs: Vec::new(),
            clip: None,
            clip_reason: None,
        };
        let config = preview::Config {
            out_dir: out.clone(),
            ..preview::Config::default()
        };
        stash_positions_to_unsent(
            &scene_with("ui", &[], None),
            &mut fresh,
            &[(0, false), (1, false), (9, false)],
            &config,
            Path::new(&staging),
        );
        let unsent = fresh.captures[0].artifact.unsent_path(&out);
        assert_eq!(fresh.captures[0].path, unsent);
        assert!(unsent.is_file());
        assert!(!changed_path.exists());
        assert_eq!(
            fresh.captures[1].path,
            fresh.captures[1].artifact.delivered_path(&out)
        );
        assert!(!same_path.exists());
    }
}
