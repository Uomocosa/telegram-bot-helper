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

    #[test]
    fn test_usage() {
        let _ = status_word;
    }
}
