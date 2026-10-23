#[must_use]
pub fn fingerprint_of(scene: &str, label: &str) -> String {
    let mut fingerprint = String::from(scene);
    fingerprint.push('|');
    fingerprint.push_str(label);
    fingerprint
}

#[cfg(test)]
mod tests {
    use super::fingerprint_of;

    #[test]
    fn test_usage() {
        assert_eq!(fingerprint_of("ui", "rest"), "ui|rest");
    }
}
