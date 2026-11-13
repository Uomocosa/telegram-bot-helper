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

    #[test]
    fn test_usage() {
        let fresh = deliver::FreshDelivery {
            captures: Vec::new(),
            clip: None,
        };
        assert_eq!(chunk_refs(&fresh, &[(0, false), (0, true)]).len(), 0);
    }
}
