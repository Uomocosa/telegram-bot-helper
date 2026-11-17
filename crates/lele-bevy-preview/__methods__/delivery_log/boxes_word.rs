use crate::deliver;

const SHOWN_BOXES: usize = 8;

#[must_use]
pub fn boxes_word(diff: &deliver::PixelDiff) -> String {
    let mut ordered: Vec<&deliver::ChangedRegion> = diff.regions.iter().collect();
    ordered.sort_by_key(|region| std::cmp::Reverse(region.pixels));
    let mut parts = Vec::new();
    for region in ordered.iter().take(SHOWN_BOXES) {
        let kind = if region.is_drift { "drift" } else { "REAL" };
        parts.push(format!(
            "({},{1} {2}x{3} {kind})",
            region.x, region.y, region.width, region.height
        ));
    }
    let rest = ordered.len().saturating_sub(SHOWN_BOXES);
    if rest > 0 {
        parts.push(format!("+{rest} more"));
    }
    format!("[{}]", parts.join(","))
}

#[cfg(test)]
mod tests {
    use super::boxes_word;
    use crate::deliver;

    #[test]
    fn test_usage() {
        let diff = deliver::PixelDiff {
            changed_pixels: 118,
            regions: vec![deliver::ChangedRegion {
                x: 120,
                y: 40,
                width: 18,
                height: 6,
                pixels: 100,
                is_drift: true,
            }],
        };
        assert_eq!(boxes_word(&diff), "[(120,40 18x6 drift)]");
    }
}
