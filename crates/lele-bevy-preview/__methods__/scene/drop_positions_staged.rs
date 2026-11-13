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
    use std::path::{Path, PathBuf};

    use super::drop_positions_staged;
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
        let shot = staging.join("ui__rest.png");
        let clip_shot = staging.join("ui__clip.png");
        std::fs::write(&shot, b"shot").unwrap();
        std::fs::write(&clip_shot, b"clip").unwrap();
        let mut fresh = deliver::FreshDelivery {
            captures: vec![capture(shot.clone(), "rest", deliver::Status::Changed)],
            clip: Some(capture(clip_shot.clone(), "clip", deliver::Status::Changed)),
        };
        let unsent = fresh.captures[0].artifact.unsent_path(&out);
        std::fs::create_dir_all(unsent.parent().unwrap()).unwrap();
        std::fs::write(&unsent, b"stale").unwrap();
        let config = preview::Config {
            out_dir: out.clone(),
            ..preview::Config::default()
        };
        drop_positions_staged(
            &scene_named("ui"),
            &mut fresh,
            &[(0, false), (0, true)],
            &config,
            Path::new(&staging),
        );
        assert!(!shot.exists());
        assert!(!clip_shot.exists());
        assert!(!unsent.exists());
        assert_eq!(
            fresh.captures[0].path,
            fresh.captures[0].artifact.delivered_path(&out)
        );
        let clip = fresh.clip.as_ref().unwrap();
        assert_eq!(clip.path, clip.artifact.delivered_path(&out));
    }
}
