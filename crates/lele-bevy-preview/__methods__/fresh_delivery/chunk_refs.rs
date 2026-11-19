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
    use super::chunk_refs;
    use crate::deliver;
    use crate::testing::capture;

    #[test]
    fn test_usage() {
        let fresh = deliver::FreshDelivery {
            captures: vec![
                capture("a.png", "a", deliver::Media::Png, deliver::Status::New),
                capture("b.png", "b", deliver::Media::Png, deliver::Status::New),
            ],
            reasons: vec![deliver::Reason::NoBaseline, deliver::Reason::NoBaseline],
            diffs: Vec::new(),
            clip: Some(capture(
                "clip.png",
                "clip",
                deliver::Media::Png,
                deliver::Status::New,
            )),
            clip_reason: Some(deliver::Reason::NoBaseline),
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
