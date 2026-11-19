use crate::deliver;

#[must_use]
pub const fn status_word(capture: &deliver::Capture) -> &'static str {
    let status = capture.status;
    match status {
        deliver::Status::New => "new",
        deliver::Status::Changed => "changed",
        deliver::Status::Same => "same",
    }
}

#[cfg(test)]
mod tests {
    use super::status_word;
    use crate::deliver;
    use crate::testing::capture;

    #[test]
    fn test_usage() {
        assert_eq!(
            status_word(&capture(
                "shot.png",
                "rest",
                deliver::Media::Png,
                deliver::Status::New
            )),
            "new"
        );
        assert_eq!(
            status_word(&capture(
                "shot.png",
                "rest",
                deliver::Media::Png,
                deliver::Status::Changed
            )),
            "changed"
        );
        assert_eq!(
            status_word(&capture(
                "shot.png",
                "rest",
                deliver::Media::Png,
                deliver::Status::Same
            )),
            "same"
        );
    }
}
