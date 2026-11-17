use crate::deliver;

pub fn record_decision(
    log: &deliver::DeliveryLog,
    capture: &deliver::Capture,
    reason: deliver::Reason,
    diff: Option<&deliver::PixelDiff>,
) {
    let media = match capture.artifact.media {
        deliver::Media::Png => "png",
        deliver::Media::Mp4 => "mp4",
    };
    let mut fields = vec![
        ("label", capture.artifact.label.clone()),
        ("status", capture.status_word().to_string()),
        ("reason", reason.word().to_string()),
        ("media", media.to_string()),
    ];
    if let Some(seen) = diff {
        fields.push(("changed_px", seen.changed_pixels.to_string()));
        fields.push(("regions", seen.regions.len().to_string()));
        fields.push(("boxes", deliver::DeliveryLog::boxes_word(seen)));
    }
    log.append_line("decision", &fields);
}

#[cfg(test)]
mod tests {
    use super::record_decision;
    use crate::deliver;
    use crate::scene;

    fn capture(label: &str, status: deliver::Status) -> deliver::Capture {
        deliver::Capture {
            path: std::path::PathBuf::from("out/ui__one.png"),
            artifact: deliver::Artifact {
                scene: String::from("ui"),
                label: label.to_string(),
                fingerprint: deliver::Artifact::fingerprint_of("ui", label),
                pixel_hash: String::from("h"),
                media: deliver::Media::Png,
                kind: scene::Kind::System,
            },
            status,
        }
    }

    #[test]
    fn test_usage() {
        let dir = tempfile::tempdir().unwrap();
        let log = deliver::DeliveryLog::from_out_dir_and_scene(dir.path(), "ui");
        let seen = capture("one room", deliver::Status::Same);
        record_decision(&log, &seen, deliver::Reason::DriftWithinTolerance, None);
        let text = std::fs::read_to_string(&log.path).unwrap();
        assert!(text.contains("event=decision"));
        assert!(text.contains("label=one room"));
        assert!(text.contains("status=same"));
        assert!(text.contains("reason=drift_within_tolerance"));
        assert!(text.contains("media=png"));
        assert!(!text.contains("changed_px="));
        let diff = deliver::PixelDiff {
            changed_pixels: 118,
            regions: vec![deliver::ChangedRegion {
                x: 120,
                y: 40,
                width: 18,
                height: 6,
                pixels: 100,
                is_drift: true,
            }],
        };
        assert_eq!(
            deliver::DeliveryLog::boxes_word(&diff),
            "[(120,40 18x6 drift)]"
        );
    }
}
