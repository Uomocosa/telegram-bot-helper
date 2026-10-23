use crate::Caption;

const MAX_CHARS: usize = 1000;

#[must_use]
pub fn truncated(caption: &Caption) -> String {
    let text: &str = caption;
    if text.chars().count() <= MAX_CHARS {
        return text.to_string();
    }
    let kept: String = text.chars().take(MAX_CHARS.saturating_sub(3)).collect();
    format!("{kept}...")
}

#[cfg(test)]
mod tests {
    use super::truncated;
    use crate::Caption;

    #[test]
    fn test_usage() {
        assert_eq!(truncated(&Caption::from("short")), "short");
        let long = "x".repeat(2000);
        let cut = truncated(&Caption::from(long.as_str()));
        assert_eq!(cut.chars().count(), 1000);
        assert!(cut.ends_with("..."));
    }
}
