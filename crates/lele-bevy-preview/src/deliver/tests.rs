#[cfg(test)]
mod tests {
    use crate::deliver;
    use crate::preview;
    use crate::scene;
    use std::path::PathBuf;

    fn artifact(scene: &str, label: &str, pixels: &str) -> deliver::Artifact {
        deliver::Artifact {
            scene: scene.to_string(),
            label: label.to_string(),
            fingerprint: deliver::Artifact::fingerprint_of(scene, label),
            pixel_hash: pixels.to_string(),
            media: deliver::Media::Png,
            kind: scene::Kind::System,
        }
    }

    fn capture(label: &str, status: deliver::Status) -> deliver::Capture {
        deliver::Capture {
            path: PathBuf::from(format!("out/{label}.png")),
            artifact: artifact("ui", label, "hash"),
            status,
        }
    }

    fn write_preview(dir: &std::path::Path, artifact: &deliver::Artifact) -> PathBuf {
        let path = dir.join(artifact.file_name());
        std::fs::write(&path, b"bytes").unwrap();
        path
    }

    fn record(dir: &std::path::Path, artifact: &deliver::Artifact) {
        write_preview(dir, artifact);
        deliver::SentPreviewHash::record(dir, artifact).unwrap();
    }

    #[test]
    fn test_usage() {
        assert_eq!(
            deliver::Artifact::slug("hover then press"),
            "hover_then_press"
        );
        assert_eq!(deliver::Artifact::slug("A-B.c"), "a_b_c");
        assert_eq!(deliver::Artifact::fingerprint_of("ui", "rest"), "ui|rest");
        assert_eq!(
            deliver::Artifact::hash_bytes(b"abc"),
            deliver::Artifact::hash_bytes(b"abc")
        );
        assert_ne!(
            deliver::Artifact::hash_bytes(b"abc"),
            deliver::Artifact::hash_bytes(b"abd")
        );
        assert_eq!(
            deliver::Artifact::hash_bytes(b"abc").len(),
            deliver::basic::constants::HASH_CHARS
        );
        assert!(capture("rest", deliver::Status::Changed).is_sendable());
        assert!(capture("rest", deliver::Status::New).is_sendable());
        assert!(!capture("rest", deliver::Status::Same).is_sendable());
    }

    #[test]
    fn test_usage_missing_sent_is_new() {
        let dir = tempfile::tempdir().unwrap();
        let current = artifact("ui", "rest", "0123456789abcdef");
        write_preview(dir.path(), &current);
        assert_eq!(
            current.status_in_cache(dir.path()).unwrap(),
            deliver::Status::New
        );
    }

    #[test]
    fn test_usage_missing_preview_file_is_new_even_with_sent() {
        let dir = tempfile::tempdir().unwrap();
        let old = artifact("ui", "rest", "0123456789abcdef");
        record(dir.path(), &old);
        std::fs::remove_file(dir.path().join(old.file_name())).unwrap();
        let current = artifact("ui", "rest", "0123456789abcdef");
        assert_eq!(
            current.status_in_cache(dir.path()).unwrap(),
            deliver::Status::New
        );
    }

    #[test]
    fn test_usage_same_hash_is_same() {
        let dir = tempfile::tempdir().unwrap();
        let current = artifact("ui", "rest", "0123456789abcdef");
        record(dir.path(), &current);
        assert_eq!(
            current.status_in_cache(dir.path()).unwrap(),
            deliver::Status::Same
        );
    }

    #[test]
    fn test_usage_different_hash_is_changed() {
        let dir = tempfile::tempdir().unwrap();
        let old = artifact("ui", "rest", "0000000000000000");
        record(dir.path(), &old);
        let current = artifact("ui", "rest", "1111111111111111");
        write_preview(dir.path(), &current);
        assert_eq!(
            current.status_in_cache(dir.path()).unwrap(),
            deliver::Status::Changed
        );
    }

    #[test]
    fn test_usage_invalid_sent_fails_and_leaves_file_untouched() {
        let dir = tempfile::tempdir().unwrap();
        let current = artifact("ui", "rest", "0123456789abcdef");
        write_preview(dir.path(), &current);
        let sent = deliver::SentPreviewHash::sent_path(dir.path(), &current);
        std::fs::create_dir_all(sent.parent().unwrap()).unwrap();
        std::fs::write(&sent, b"not-a-hash").unwrap();
        let before = std::fs::read(&sent).unwrap();
        let result = current.status_in_cache(dir.path());
        assert!(matches!(result, Err(crate::Error::SentCacheInvalid { .. })));
        let after = std::fs::read(&sent).unwrap();
        assert_eq!(before, after);
    }

    #[test]
    fn test_usage_collect_labels_every_artifact_with_a_status() {
        let dir = tempfile::tempdir().unwrap();
        let rendered = vec![preview::RenderedState {
            path: dir.path().join("ui__rest.png"),
            label: String::from("rest"),
            pixel_hash: String::from("0123456789abcdef"),
        }];
        std::fs::write(&rendered[0].path, b"bytes").unwrap();
        let captures =
            deliver::Capture::collect("ui", scene::Kind::System, &rendered, dir.path()).unwrap();
        assert_eq!(captures.len(), 1);
        assert_eq!(captures[0].status, deliver::Status::New);
        assert_eq!(captures[0].artifact.fingerprint, "ui|rest");
        deliver::SentPreviewHash::record(dir.path(), &captures[0].artifact).unwrap();
        let again =
            deliver::Capture::collect("ui", scene::Kind::System, &rendered, dir.path()).unwrap();
        assert_eq!(again[0].status, deliver::Status::Same);
    }

    #[test]
    fn test_usage_hash_file_reads_bytes_and_reports_missing_files() {
        let dir = tempfile::tempdir().unwrap();
        let png = dir.path().join("bytes.png");
        std::fs::write(&png, b"pixel-bytes").unwrap();
        assert_eq!(
            deliver::Artifact::hash_file(&png).unwrap(),
            deliver::Artifact::hash_bytes(b"pixel-bytes")
        );
        assert!(deliver::Artifact::hash_file(&dir.path().join("absent.png")).is_err());
    }

    #[test]
    fn test_usage_sent_path_uses_file_name_in_cache() {
        let dir = tempfile::tempdir().unwrap();
        let current = artifact("lobby_ui", "one room", "0123456789abcdef");
        let path = deliver::SentPreviewHash::sent_path(dir.path(), &current);
        assert!(path.ends_with("lobby_ui__one_room.png.sent"));
        assert!(
            path.to_string_lossy()
                .contains(deliver::basic::constants::CACHE_DIR)
        );
    }

    #[test]
    fn test_usage_record_writes_hash_with_newline_atomically() {
        let dir = tempfile::tempdir().unwrap();
        let current = artifact("ui", "rest", "0123456789abcdef");
        write_preview(dir.path(), &current);
        deliver::SentPreviewHash::record(dir.path(), &current).unwrap();
        let path = deliver::SentPreviewHash::sent_path(dir.path(), &current);
        let text = std::fs::read_to_string(&path).unwrap();
        assert_eq!(text, "0123456789abcdef\n");
        assert_eq!(text.trim_end().len(), deliver::basic::constants::HASH_CHARS);
    }
    // no test_usage necessary
}
