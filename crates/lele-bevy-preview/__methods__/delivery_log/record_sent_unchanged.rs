use crate::deliver;

pub fn record_sent_unchanged(log: &deliver::DeliveryLog, capture: &deliver::Capture, why: &str) {
    log.append_line(
        "sent_unchanged",
        &[
            ("label", capture.artifact.label.clone()),
            ("why", why.to_string()),
        ],
    );
}

#[cfg(test)]
mod tests {
    use super::record_sent_unchanged;
    use crate::deliver;
    use crate::scene;

    #[test]
    fn test_usage() {
        let dir = tempfile::tempdir().unwrap();
        let log = deliver::DeliveryLog::from_out_dir_and_scene(dir.path(), "ui");
        let capture = deliver::Capture {
            path: std::path::PathBuf::from("out/ui__one.png"),
            artifact: deliver::Artifact {
                scene: String::from("ui"),
                label: String::from("one"),
                fingerprint: deliver::Artifact::fingerprint_of("ui", "one"),
                pixel_hash: String::from("h"),
                media: deliver::Media::Png,
                kind: scene::Kind::System,
            },
            status: deliver::Status::Same,
        };
        record_sent_unchanged(&log, &capture, "album_with_changed_sibling");
        let text = std::fs::read_to_string(&log.path).unwrap();
        assert!(text.contains("event=sent_unchanged"));
        assert!(text.contains("why=album_with_changed_sibling"));
    }
}
