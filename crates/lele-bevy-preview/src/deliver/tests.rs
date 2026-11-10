#[cfg(test)]
mod tests {
    use crate::deliver;
    use crate::preview;
    use crate::scene;
    use std::path::PathBuf;

    fn solid_png(path: &std::path::Path, red: u8, green: u8, blue: u8) -> String {
        let image = image::RgbaImage::from_pixel(8, 8, image::Rgba([red, green, blue, 255]));
        image.save(path).unwrap();
        deliver::Artifact::hash_png_pixels(path).unwrap()
    }

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
            deliver::HASH_CHARS
        );
        assert!(capture("rest", deliver::Status::Changed).is_sendable());
        assert!(capture("rest", deliver::Status::New).is_sendable());
        assert!(!capture("rest", deliver::Status::Same).is_sendable());
    }

    #[test]
    fn test_usage_missing_delivered_is_new() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("ui__rest.png");
        let hash = solid_png(&path, 10, 12, 16);
        let current = artifact("ui", "rest", &hash);
        std::fs::remove_file(&path).unwrap();
        assert_eq!(
            current.status_against_baseline(dir.path()).unwrap(),
            deliver::Status::New
        );
    }

    #[test]
    fn test_usage_missing_preview_file_is_new() {
        let dir = tempfile::tempdir().unwrap();
        let current = artifact("ui", "rest", "0123456789abcdef");
        assert_eq!(
            current.status_against_baseline(dir.path()).unwrap(),
            deliver::Status::New
        );
    }

    #[test]
    fn test_usage_same_pixels_is_same() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("ui__rest.png");
        let hash = solid_png(&path, 20, 30, 40);
        let current = artifact("ui", "rest", &hash);
        assert_eq!(
            current.status_against_baseline(dir.path()).unwrap(),
            deliver::Status::Same
        );
    }

    #[test]
    fn test_usage_different_pixels_is_changed() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("ui__rest.png");
        solid_png(&path, 1, 2, 3);
        let staged = dir.path().join("staged.png");
        let staged_hash = solid_png(&staged, 4, 5, 6);
        let current = artifact("ui", "rest", &staged_hash);
        assert_eq!(
            current.status_against_baseline(dir.path()).unwrap(),
            deliver::Status::Changed
        );
    }

    #[test]
    fn test_usage_corrupt_delivered_fails_and_leaves_file_untouched() {
        let dir = tempfile::tempdir().unwrap();
        let current = artifact("ui", "rest", "0123456789abcdef");
        let path = dir.path().join(current.file_name());
        std::fs::write(&path, b"not-a-png").unwrap();
        let before = std::fs::read(&path).unwrap();
        let result = current.status_against_baseline(dir.path());
        assert!(result.is_err());
        let after = std::fs::read(&path).unwrap();
        assert_eq!(before, after);
    }

    #[test]
    fn test_usage_collect_labels_every_artifact_with_a_status() {
        let dir = tempfile::tempdir().unwrap();
        let staging = dir.path().join("staging").join("ui__rest.png");
        std::fs::create_dir_all(staging.parent().unwrap()).unwrap();
        let hash = solid_png(&staging, 7, 8, 9);
        let rendered = vec![preview::RenderedState {
            path: staging.clone(),
            label: String::from("rest"),
            pixel_hash: hash,
        }];
        let captures =
            deliver::Capture::collect("ui", scene::Kind::System, &rendered, dir.path()).unwrap();
        assert_eq!(captures.len(), 1);
        assert_eq!(captures[0].status, deliver::Status::New);
        assert_eq!(captures[0].artifact.fingerprint, "ui|rest");
        let delivered = dir.path().join("ui__rest.png");
        std::fs::copy(&staging, &delivered).unwrap();
        let again =
            deliver::Capture::collect("ui", scene::Kind::System, &rendered, dir.path()).unwrap();
        assert_eq!(again[0].status, deliver::Status::Same);
    }

    #[test]
    fn test_usage_hash_png_pixels_reads_decoded_pixels_and_reports_missing() {
        let dir = tempfile::tempdir().unwrap();
        let png = dir.path().join("bytes.png");
        let hash = solid_png(&png, 11, 22, 33);
        assert_eq!(deliver::Artifact::hash_png_pixels(&png).unwrap(), hash);
        assert!(deliver::Artifact::hash_png_pixels(&dir.path().join("absent.png")).is_err());
    }

    #[test]
    fn test_usage_paths_use_file_name_in_dirs() {
        let dir = tempfile::tempdir().unwrap();
        let current = artifact("lobby_ui", "one room", "0123456789abcdef");
        let delivered = current.delivered_path(dir.path());
        assert!(delivered.ends_with("lobby_ui__one_room.png"));
        let unsent = current.unsent_path(dir.path());
        assert!(unsent.to_string_lossy().contains("_unsent"));
        assert!(unsent.ends_with("lobby_ui__one_room.png"));
        let locks = deliver::Artifact::locks_dir(dir.path());
        assert!(locks.ends_with("_locks"));
        let staging = deliver::Artifact::staging_root(dir.path(), "lobby_ui");
        assert!(staging.to_string_lossy().contains("_staging"));
    }

    #[test]
    fn test_usage_delivered_and_unsent_frames_dirs() {
        let dir = tempfile::tempdir().unwrap();
        assert!(deliver::Artifact::delivered_frames_dir(dir.path(), "ui").ends_with("ui_frames"));
        assert!(
            deliver::Artifact::unsent_frames_dir(dir.path(), "ui")
                .to_string_lossy()
                .contains("_unsent")
        );
        let staging = deliver::Artifact::staging_root(dir.path(), "ui");
        assert!(deliver::Artifact::staging_frames_dir(&staging, "ui").ends_with("ui_frames"));
    }
    // no test_usage necessary
}
