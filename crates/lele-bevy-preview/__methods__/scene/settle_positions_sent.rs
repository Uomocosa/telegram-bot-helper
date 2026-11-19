use std::path::Path;

use crate::deliver;
use crate::preview;
use crate::scene;

pub fn settle_positions_sent(
    scene: &scene::Scene,
    fresh: &mut deliver::FreshDelivery,
    positions: &[(usize, bool)],
    config: &preview::Config,
    staging_root: &Path,
    resend_all: bool,
) {
    for (position, is_clip) in positions {
        if *is_clip {
            if let Some(clip) = fresh.clip.as_mut() {
                let _ = clip.promote_staged_to_delivered(
                    &config.out_dir,
                    &scene.name,
                    staging_root,
                    resend_all,
                );
            }
        } else if let Some(capture) = fresh.captures.get_mut(*position) {
            let _ = capture.promote_staged_to_delivered(
                &config.out_dir,
                &scene.name,
                staging_root,
                resend_all,
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use std::path::{Path, PathBuf};

    use super::settle_positions_sent;
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
                    changed_path,
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
        settle_positions_sent(
            &scene_with("ui", &[], None),
            &mut fresh,
            &[(0, false), (1, false), (9, false)],
            &config,
            Path::new(&staging),
            false,
        );
        let rest_delivered = fresh.captures[0].artifact.delivered_path(&out);
        assert_eq!(fresh.captures[0].path, rest_delivered);
        assert_eq!(std::fs::read(&rest_delivered).unwrap(), b"changed");
        assert_eq!(
            fresh.captures[1].path,
            fresh.captures[1].artifact.delivered_path(&out)
        );
        assert!(!same_path.exists());

        let resend_staging = dir.path().join("resend");
        std::fs::create_dir_all(&resend_staging).unwrap();
        let resent_path: PathBuf = resend_staging.join("ui__idle.png");
        std::fs::write(&resent_path, b"resent").unwrap();
        let mut resend = deliver::FreshDelivery {
            captures: vec![capture(
                resent_path,
                "idle",
                deliver::Media::Png,
                deliver::Status::Same,
            )],
            reasons: vec![deliver::Reason::DriftWithinTolerance],
            diffs: Vec::new(),
            clip: None,
            clip_reason: None,
        };
        settle_positions_sent(
            &scene_with("ui", &[], None),
            &mut resend,
            &[(0, false)],
            &config,
            &resend_staging,
            true,
        );
        let idle_delivered = resend.captures[0].artifact.delivered_path(&out);
        assert_eq!(std::fs::read(&idle_delivered).unwrap(), b"resent");
    }
}
