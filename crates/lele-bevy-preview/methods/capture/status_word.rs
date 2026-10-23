use crate::deliver;
use crate::deliver::basic::enums::Status;

#[must_use]
pub const fn status_word(capture: &deliver::Capture) -> &'static str {
    let status = capture.status;
    match status {
        Status::New => "new",
        Status::Changed => "changed",
        Status::Same => "same",
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
