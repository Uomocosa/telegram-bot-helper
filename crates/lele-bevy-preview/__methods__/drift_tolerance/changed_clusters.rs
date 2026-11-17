use crate::deliver;

#[must_use]
pub fn changed_clusters(
    tolerance: &deliver::DriftTolerance,
    mask: &[bool],
    width: u32,
    height: u32,
) -> Vec<Vec<(u32, u32)>> {
    let mut visited = vec![false; mask.len()];
    let mut clusters: Vec<Vec<(u32, u32)>> = Vec::new();
    let gap = i64::from(tolerance.cluster_gap);
    let min_gap = gap.checked_neg().unwrap_or(0);
    for y in 0..height {
        for x in 0..width {
            let start = deliver::DriftTolerance::pixel_index(width, x, y);
            let is_unvisited_change = mask.get(start).copied().unwrap_or(false)
                && !visited.get(start).copied().unwrap_or(true);
            if !is_unvisited_change {
                continue;
            }
            let mut pending = vec![(x, y)];
            let mut cluster = Vec::new();
            while let Some((current_x, current_y)) = pending.pop() {
                let index = deliver::DriftTolerance::pixel_index(width, current_x, current_y);
                if visited.get(index).copied().unwrap_or(true)
                    || !mask.get(index).copied().unwrap_or(false)
                {
                    continue;
                }
                if let Some(slot) = visited.get_mut(index) {
                    *slot = true;
                }
                cluster.push((current_x, current_y));
                for delta_y in min_gap..=gap {
                    for delta_x in min_gap..=gap {
                        let neighbour_x = i64::from(current_x).checked_add(delta_x);
                        let neighbour_y = i64::from(current_y).checked_add(delta_y);
                        if let (Some(Ok(next_x)), Some(Ok(next_y))) = (
                            neighbour_x.map(u32::try_from),
                            neighbour_y.map(u32::try_from),
                        ) && next_x < width
                            && next_y < height
                        {
                            pending.push((next_x, next_y));
                        }
                    }
                }
            }
            if !cluster.is_empty() {
                clusters.push(cluster);
            }
        }
    }
    clusters
}

#[cfg(test)]
mod tests {
    use super::changed_clusters;
    use crate::deliver;

    #[test]
    fn test_usage() {
        let tolerance = deliver::DriftTolerance::default();
        let mask = vec![true, false, false, true];
        let clusters = changed_clusters(&tolerance, &mask, 4, 1);
        assert_eq!(clusters.len(), 2);
        assert_eq!(clusters[0], vec![(0, 0)]);
        assert_eq!(clusters[1], vec![(3, 0)]);
        assert_eq!(changed_clusters(&tolerance, &[], 0, 0).len(), 0);
    }
}
