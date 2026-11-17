#[must_use]
pub fn sanitized_field(value: &str) -> String {
    let mut cleaned = String::with_capacity(value.len());
    for character in value.chars() {
        if character == '\t' || character == '\n' || character == '\r' {
            cleaned.push(' ');
        } else {
            cleaned.push(character);
        }
    }
    cleaned
}

#[cfg(test)]
mod tests {
    use super::sanitized_field;

    #[test]
    fn test_usage() {
        assert_eq!(
            sanitized_field("one\ttwo\nthree\rfour"),
            "one two three four"
        );
        assert_eq!(sanitized_field("plain"), "plain");
    }
}
