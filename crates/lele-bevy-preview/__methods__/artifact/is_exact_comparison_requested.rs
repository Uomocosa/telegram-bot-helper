#[must_use]
pub fn is_exact_comparison_requested() -> bool {
    let Ok(flag) = std::env::var("LELE_PREVIEW_EXACT") else {
        return false;
    };
    flag == "1" || flag.eq_ignore_ascii_case("true")
}

#[cfg(test)]
mod tests {
    use super::is_exact_comparison_requested;

    #[test]
    fn test_usage() {
        unsafe { std::env::set_var("LELE_PREVIEW_EXACT", "true") };
        assert!(is_exact_comparison_requested());
        unsafe { std::env::set_var("LELE_PREVIEW_EXACT", "0") };
        assert!(!is_exact_comparison_requested());
        unsafe { std::env::remove_var("LELE_PREVIEW_EXACT") };
    }
}
