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
    use super::chunk_is_sendable;
    use crate::deliver;
    use crate::testing::capture;

    #[test]
    fn test_usage() {
        let fresh = deliver::FreshDelivery {
            captures: vec![capture(
                "idle.png",
                "idle",
                deliver::Media::Png,
                deliver::Status::Same,
            )],
            reasons: vec![deliver::Reason::DriftWithinTolerance],
            diffs: Vec::new(),
            clip: Some(capture(
                "clip.png",
                "clip",
                deliver::Media::Png,
                deliver::Status::Changed,
            )),
            clip_reason: Some(deliver::Reason::ClustersExceedDrift),
        };
        assert!(!chunk_is_sendable(&fresh, &[(0, false)]));
        assert!(chunk_is_sendable(&fresh, &[(0, false), (0, true)]));
        assert!(!chunk_is_sendable(&fresh, &[(9, false)]));
    }
}
