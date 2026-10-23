use atomic_delegate_macros::atomic_delegates;
use derive_more::{Deref, From};

#[derive(Deref, From)]
pub struct Caption<'a>(pub &'a str);

#[atomic_delegates]
impl Caption<'_> {
    #[must_use]
    pub fn truncated(&self) -> String {}
}

#[cfg(test)]
mod tests {
    use super::Caption;

    #[test]
    fn test_usage() {
        assert_eq!(Caption::from("short").truncated(), "short");
        let long = "x".repeat(2000);
        let cut = Caption::from(long.as_str()).truncated();
        assert_eq!(cut.chars().count(), 1000);
        assert!(cut.ends_with("..."));
        let emoji = "✅".repeat(2000);
        let cut_emoji = Caption::from(emoji.as_str()).truncated();
        assert_eq!(cut_emoji.chars().count(), 1000);
    }
}
