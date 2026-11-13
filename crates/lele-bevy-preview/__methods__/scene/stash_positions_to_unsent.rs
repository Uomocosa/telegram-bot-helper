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
    use std::path::{Path, PathBuf};

    use super::stash_positions_to_unsent;
    use crate::deliver;
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

    fn capture(path: PathBuf, label: &str, status: deliver::Status) -> deliver::Capture {
        deliver::Capture {
            path,
            artifact: deliver::Artifact {
                scene: String::from("ui"),
                label: String::from(label),
                fingerprint: deliver::Artifact::fingerprint_of("ui", label),
                pixel_hash: String::from("hash"),
                media: deliver::Media::Png,
                kind: scene::Kind::System,
            },
            status,
        }
    }

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
                capture(changed_path.clone(), "rest", deliver::Status::Changed),
                capture(same_path.clone(), "idle", deliver::Status::Same),
            ],
            clip: None,
        };
        let config = preview::Config {
            out_dir: out.clone(),
            ..preview::Config::default()
        };
        stash_positions_to_unsent(
            &scene_named("ui"),
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
