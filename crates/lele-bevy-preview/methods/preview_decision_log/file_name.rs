use crate::deliver::basic::constants;

#[must_use]
pub fn file_name() -> String {
    let name = constants::LOG_FILE;
    String::from(name)
}

#[cfg(test)]
mod tests {
    use super::file_name;

    #[test]
    fn test_usage() {
        assert_ne!(file_name().len(), 0);
    }
}
