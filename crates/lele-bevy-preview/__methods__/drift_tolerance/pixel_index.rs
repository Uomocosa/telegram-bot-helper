#[must_use]
pub fn pixel_index(width: u32, x: u32, y: u32) -> usize {
    usize::try_from(y)
        .unwrap_or_default()
        .saturating_mul(usize::try_from(width).unwrap_or_default())
        .saturating_add(usize::try_from(x).unwrap_or_default())
}

#[cfg(test)]
mod tests {
    use super::pixel_index;

    #[test]
    fn test_usage() {
        assert_eq!(pixel_index(4, 2, 3), 14);
        assert_eq!(pixel_index(4, 0, 0), 0);
    }
}
