#[must_use]
pub fn is_resend_all_requested() -> bool {
    let Ok(flag) = std::env::var("LELE_PREVIEW_RESEND_ALL") else {
        return false;
    };
    flag == "1" || flag.eq_ignore_ascii_case("true")
}

#[cfg(test)]
mod tests {
    use super::is_resend_all_requested;

    #[test]
    fn test_usage() {
        unsafe { std::env::set_var("LELE_PREVIEW_RESEND_ALL", "1") };
        assert!(is_resend_all_requested());
        unsafe { std::env::set_var("LELE_PREVIEW_RESEND_ALL", "false") };
        assert!(!is_resend_all_requested());
        unsafe { std::env::remove_var("LELE_PREVIEW_RESEND_ALL") };
    }
}
