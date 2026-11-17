pub fn days_to_ymd(days: u64) -> (i64, u32, u32) {
    let shifted = days.saturating_add(719_468);
    let era = shifted.checked_div(146_097).unwrap_or(0);
    let day_of_era = shifted.checked_rem(146_097).unwrap_or(0);
    let year_removed = day_of_era
        .checked_div(1_460)
        .unwrap_or(0)
        .saturating_add(day_of_era.checked_div(36_524).unwrap_or(0))
        .saturating_sub(day_of_era.checked_div(146_096).unwrap_or(0));
    let year_of_era = day_of_era
        .saturating_sub(year_removed)
        .checked_div(365)
        .unwrap_or(0);
    let year = year_of_era.saturating_add(era.saturating_mul(400));
    let day_of_year = day_of_era.saturating_sub(
        year_of_era
            .saturating_mul(365)
            .saturating_add(year_of_era.checked_div(4).unwrap_or(0))
            .saturating_sub(year_of_era.checked_div(100).unwrap_or(0)),
    );
    let month_part = day_of_year
        .saturating_mul(5)
        .saturating_add(2)
        .checked_div(153)
        .unwrap_or(0);
    let day = day_of_year.saturating_sub(
        month_part
            .saturating_mul(153)
            .saturating_add(2)
            .checked_div(5)
            .unwrap_or(0),
    );
    let month = if month_part < 10 {
        month_part.saturating_add(3)
    } else {
        month_part.saturating_sub(9)
    };
    let full_year: i64 = if month <= 2 {
        i64::try_from(year).unwrap_or(0).saturating_add(1)
    } else {
        i64::try_from(year).unwrap_or(0)
    };
    let day_number = u32::try_from(day.saturating_add(1)).unwrap_or(1);
    let month_number = u32::try_from(month).unwrap_or(1);
    (full_year, month_number, day_number)
}

#[cfg(test)]
mod tests {
    use super::days_to_ymd;

    #[test]
    fn test_usage() {
        assert_eq!(days_to_ymd(0), (1970, 1, 1));
        assert_eq!(days_to_ymd(20_475), (2026, 1, 22));
    }
}
