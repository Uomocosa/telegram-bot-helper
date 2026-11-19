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
    use super::all_positions;
    use crate::deliver;
    use crate::testing::capture;

    #[test]
    fn test_usage() {
        let with_clip = deliver::FreshDelivery {
            captures: vec![
                capture("shot.png", "a", deliver::Media::Png, deliver::Status::New),
                capture("shot.png", "b", deliver::Media::Png, deliver::Status::New),
            ],
            reasons: vec![deliver::Reason::NoBaseline, deliver::Reason::NoBaseline],
            diffs: Vec::new(),
            clip: Some(capture(
                "shot.png",
                "clip",
                deliver::Media::Png,
                deliver::Status::New,
            )),
            clip_reason: Some(deliver::Reason::NoBaseline),
        };
        assert_eq!(
            all_positions(&with_clip),
            vec![(0, false), (1, false), (0, true)]
        );
        let empty = deliver::FreshDelivery {
            captures: Vec::new(),
            reasons: Vec::new(),
            diffs: Vec::new(),
            clip: None,
            clip_reason: None,
        };
        assert_eq!(all_positions(&empty), Vec::new());
    }
}
