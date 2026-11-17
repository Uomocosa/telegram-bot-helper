use crate::deliver;

pub fn utc_timestamp() -> String {
    let seconds = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(0, |elapsed| elapsed.as_secs());
    let days = seconds.checked_div(86_400).unwrap_or(0);
    let clock = seconds.checked_rem(86_400).unwrap_or(0);
    let hour = clock.checked_div(3_600).unwrap_or(0);
    let minute = clock
        .checked_rem(3_600)
        .unwrap_or(0)
        .checked_div(60)
        .unwrap_or(0);
    let second = clock.checked_rem(60).unwrap_or(0);
    let (year, month, day) = deliver::DeliveryLog::days_to_ymd(days);
    format!("{year:04}-{month:02}-{day:02}T{hour:02}:{minute:02}:{second:02}Z")
}

#[cfg(test)]
mod tests {
    use super::utc_timestamp;

    #[test]
    fn test_usage() {
        let stamp = utc_timestamp();
        assert!(stamp.ends_with('Z'));
        assert!(stamp.contains('T'));
        assert_eq!(stamp.len(), 20);
    }
}
