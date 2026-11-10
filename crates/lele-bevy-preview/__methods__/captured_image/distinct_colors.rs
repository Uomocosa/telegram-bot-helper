use std::collections::HashSet;

use crate::preview;

#[must_use]
pub fn distinct_colors(image: &preview::CapturedImage) -> usize {
    let mut seen: HashSet<u32> = HashSet::new();
    let (chunks, _) = image.bytes().as_chunks::<4>();
    for pixel in chunks {
        seen.insert(u32::from_be_bytes(*pixel));
    }
    seen.len()
}

#[cfg(test)]
mod tests {
    use super::distinct_colors;

    #[test]
    fn test_usage() {
        let _ = distinct_colors;
    }
}
