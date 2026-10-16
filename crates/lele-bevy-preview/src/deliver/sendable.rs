use crate::deliver::basic::enums::Status;

#[must_use]
pub const fn sendable(status: Status) -> bool {
    matches!(status, Status::New | Status::Changed)
}
// no test_usage necessary
