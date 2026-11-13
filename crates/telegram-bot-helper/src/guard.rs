use atomic_delegate_macros::atomic_delegates;

pub struct Guard {
    pub name: String,
    pub start: std::time::Instant,
}

impl From<&str> for Guard {
    fn from(name: &str) -> Self {
        Self {
            name: name.to_string(),
            start: std::time::Instant::now(),
        }
    }
}

#[atomic_delegates]
impl Drop for Guard {
    fn drop(&mut self) {}
}

#[cfg(test)]
mod tests {
    use super::Guard;

    #[test]
    fn test_usage() {
        let guard = Guard::from("guard_start_smoke");
        assert_eq!(guard.name, "guard_start_smoke");
    }
}
