use crate::deliver;

pub fn chunk_refs<'a>(
    fresh: &'a deliver::FreshDelivery,
    chunk: &[(usize, bool)],
) -> Vec<&'a deliver::Capture> {
    chunk
        .iter()
        .filter_map(|(position, is_clip)| {
            if *is_clip {
                fresh.clip.as_ref()
            } else {
                fresh.captures.get(*position)
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::chunk_refs;
    use crate::deliver;
    use crate::scene;

    fn capture(label: &str) -> deliver::Capture {
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
            status: deliver::Status::New,
        }
    }

    #[test]
    fn test_usage() {
        let fresh = deliver::FreshDelivery {
            captures: vec![capture("a"), capture("b")],
            clip: Some(capture("clip")),
        };
        let refs = chunk_refs(&fresh, &[(1, false), (0, true), (5, false)]);
        let labels: Vec<&str> = refs
            .iter()
            .map(|item| item.artifact.label.as_str())
            .collect();
        assert_eq!(labels, vec!["b", "clip"]);
        assert_eq!(chunk_refs(&fresh, &[]), Vec::<&deliver::Capture>::new());
    }
}
