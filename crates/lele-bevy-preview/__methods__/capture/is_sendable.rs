use crate::deliver;

#[must_use]
pub const fn is_sendable(capture: &deliver::Capture) -> bool {
    let status = capture.status;
    matches!(status, deliver::Status::New | deliver::Status::Changed)
}

#[cfg(test)]
mod tests {
    use super::is_sendable;
    use crate::deliver;
    use crate::testing::capture;

    #[test]
    fn test_usage() {
        assert!(is_sendable(&capture(
            "shot.png",
            "rest",
            deliver::Media::Png,
            deliver::Status::New
        )));
        assert!(is_sendable(&capture(
            "shot.png",
            "rest",
            deliver::Media::Png,
            deliver::Status::Changed
        )));
        assert!(!is_sendable(&capture(
            "shot.png",
            "rest",
            deliver::Media::Png,
            deliver::Status::Same
        )));
    }
}
