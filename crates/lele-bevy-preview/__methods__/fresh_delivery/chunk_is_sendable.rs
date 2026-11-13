use crate::deliver;

pub fn chunk_is_sendable(fresh: &deliver::FreshDelivery, chunk: &[(usize, bool)]) -> bool {
    chunk.iter().any(|(position, is_clip)| {
        if *is_clip {
            fresh
                .clip
                .as_ref()
                .is_some_and(deliver::Capture::is_sendable)
        } else {
            fresh
                .captures
                .get(*position)
                .is_some_and(deliver::Capture::is_sendable)
        }
    })
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::chunk_is_sendable;
    use crate::deliver;
    use crate::scene;

    fn capture(label: &str, status: deliver::Status) -> deliver::Capture {
        deliver::Capture {
            path: PathBuf::from(format!("{label}.png")),
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
        let fresh = deliver::FreshDelivery {
            captures: vec![capture("idle", deliver::Status::Same)],
            clip: Some(capture("clip", deliver::Status::Changed)),
        };
        assert!(!chunk_is_sendable(&fresh, &[(0, false)]));
        assert!(chunk_is_sendable(&fresh, &[(0, false), (0, true)]));
        assert!(!chunk_is_sendable(&fresh, &[(9, false)]));
    }
}
