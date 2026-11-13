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

    #[test]
    fn test_usage() {
        let fresh = deliver::FreshDelivery {
            captures: Vec::new(),
            clip: None,
        };
        assert!(!chunk_is_sendable(&fresh, &[(0, false), (0, true)]));
    }
}
