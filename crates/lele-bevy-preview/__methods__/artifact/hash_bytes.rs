use crate::deliver;

#[must_use]
pub fn hash_bytes(bytes: &[u8]) -> String {
    let hex = blake3::hash(bytes).to_hex();
    hex.chars().take(deliver::HASH_CHARS).collect()
}

#[cfg(test)]
mod tests {
    use super::hash_bytes;

    #[test]
    fn test_usage() {
        assert_eq!(hash_bytes(b"abc"), hash_bytes(b"abc"));
        assert_ne!(hash_bytes(b"abc"), hash_bytes(b"abd"));
    }
}
