#[must_use]
pub fn resend_all() -> bool {
    std::env::var("LELE_PREVIEW_RESEND_ALL")
        .is_ok_and(|value| value == "1" || value.eq_ignore_ascii_case("true"))
}
// no test_usage necessary
