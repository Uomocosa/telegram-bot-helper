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

    #[test]
    fn test_usage() {
        let _ = all_positions;
    }
}
