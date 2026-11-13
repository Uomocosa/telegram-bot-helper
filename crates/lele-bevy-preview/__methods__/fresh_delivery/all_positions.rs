use crate::deliver;

#[must_use]
pub fn all_positions(fresh: &deliver::FreshDelivery) -> Vec<(usize, bool)> {
    let mut positions = Vec::with_capacity(
        fresh
            .captures
            .len()
            .saturating_add(usize::from(fresh.clip.is_some())),
    );
    for index in 0..fresh.captures.len() {
        positions.push((index, false));
    }
    if fresh.clip.is_some() {
        positions.push((0, true));
    }
    positions
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::all_positions;
    use crate::deliver;
    use crate::scene;

    fn artifact(scene: &str, label: &str, pixel_hash: &str) -> deliver::Artifact {
        deliver::Artifact {
            scene: String::from(scene),
            label: String::from(label),
            fingerprint: deliver::Artifact::fingerprint_of(scene, label),
            pixel_hash: String::from(pixel_hash),
            media: deliver::Media::Png,
            kind: scene::Kind::System,
        }
    }

    fn capture(label: &str) -> deliver::Capture {
        deliver::Capture {
            path: PathBuf::from("shot.png"),
            artifact: artifact("ui", label, "hash"),
            status: deliver::Status::New,
        }
    }

    #[test]
    fn test_usage() {
        let with_clip = deliver::FreshDelivery {
            captures: vec![capture("a"), capture("b")],
            clip: Some(capture("clip")),
        };
        assert_eq!(
            all_positions(&with_clip),
            vec![(0, false), (1, false), (0, true)]
        );
        let empty = deliver::FreshDelivery {
            captures: Vec::new(),
            clip: None,
        };
        assert_eq!(all_positions(&empty), Vec::new());
    }
}
