use crate::deliver;
use crate::deliver::basic::enums::Status;

#[must_use]
pub const fn is_sendable(capture: &deliver::Capture) -> bool {
    let status = capture.status;
    matches!(status, Status::New | Status::Changed)
}

#[cfg(test)]
mod tests {
    use super::is_sendable;

    #[test]
    fn test_usage() {
        let _ = is_sendable;
    }
}
