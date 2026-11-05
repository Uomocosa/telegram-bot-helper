use std::path::Path;

use crate::deliver;

#[must_use]
pub fn sent_path(out_dir: &Path, artifact: &deliver::Artifact) -> std::path::PathBuf {
    let cache = deliver::SentPreviewHash::cache_dir(out_dir);
    cache.join(format!("{}.sent", artifact.file_name()))
}

#[cfg(test)]
mod tests {
    use super::sent_path;
    use crate::deliver;
    use crate::scene;

    #[test]
    fn test_usage() {
        let artifact = deliver::Artifact {
            scene: String::from("lobby_ui"),
            label: String::from("one room"),
            fingerprint: deliver::Artifact::fingerprint_of("lobby_ui", "one room"),
            pixel_hash: String::from("0123456789abcdef"),
            media: deliver::Media::Png,
            kind: scene::Kind::System,
        };
        let dir = tempfile::tempdir().unwrap();
        let path = sent_path(dir.path(), &artifact);
        assert!(path.ends_with("lobby_ui__one_room.png.sent"));
    }
}
