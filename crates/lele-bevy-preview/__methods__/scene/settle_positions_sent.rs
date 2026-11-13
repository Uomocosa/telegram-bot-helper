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
    use super::settle_positions_sent;
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
        settle_positions_sent(
            &owner,
            &mut fresh,
            &[(0, false), (0, true)],
            &config,
            Path::new("staging"),
            false,
        );
        assert_eq!(fresh.captures.len(), 0);
        assert!(fresh.clip.is_none());
    }
}
