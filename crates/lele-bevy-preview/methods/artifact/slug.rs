#[must_use]
pub fn slug(label: &str) -> String {
    let slug: String = label
        .chars()
        .map(|character| {
            if character.is_ascii_alphanumeric() {
                character.to_ascii_lowercase()
            } else {
                '_'
            }
        })
        .collect();
    slug
}

#[cfg(test)]
mod tests {
    use super::slug;

    #[test]
    fn test_usage() {
        assert_eq!(slug("One Room!"), "one_room_");
    }
}
