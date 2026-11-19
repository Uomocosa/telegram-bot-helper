#[must_use]
pub fn is_status_reporting_enabled() -> bool {
    let value = std::env::var("TELEGRAM_NOTIFY_STATUS");
    value.is_ok_and(|value| value == "1" || value.eq_ignore_ascii_case("true"))
}

#[cfg(test)]
mod tests {
    use super::is_status_reporting_enabled;

    #[test]
    fn test_usage() {
        unsafe { std::env::set_var("TELEGRAM_NOTIFY_STATUS", "1") };
        assert!(is_status_reporting_enabled());
        unsafe { std::env::set_var("TELEGRAM_NOTIFY_STATUS", "0") };
        assert!(!is_status_reporting_enabled());
        unsafe { std::env::remove_var("TELEGRAM_NOTIFY_STATUS") };
    }
}
