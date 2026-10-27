use crate::deliver;

#[must_use]
pub fn clip_hash(fps: u32, frame_hashes: &[String]) -> String {
    let joined = format!("{fps}:{}", frame_hashes.join(","));
    deliver::Artifact::hash_bytes(joined.as_bytes())
}

#[cfg(test)]
mod tests {
    use super::clip_hash;

    #[test]
    fn test_usage() {
        let frames = [String::from("a"), String::from("b")];
        assert_eq!(clip_hash(10, &frames), clip_hash(10, &frames));
        assert_ne!(clip_hash(10, &frames), clip_hash(12, &frames));
    }
}
