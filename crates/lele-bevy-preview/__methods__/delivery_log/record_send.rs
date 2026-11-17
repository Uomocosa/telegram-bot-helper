use crate::Error;
use crate::deliver;

pub fn record_send(
    log: &deliver::DeliveryLog,
    chunk_index: usize,
    items: &[&deliver::Capture],
    resend_all: bool,
    result: &Result<usize, Error>,
) {
    let mut summary = String::new();
    for (position, capture) in items.iter().enumerate() {
        if position > 0 {
            summary.push(',');
        }
        summary.push_str(&capture.artifact.label);
        summary.push(':');
        summary.push_str(capture.status_word());
    }
    let outcome = match result {
        Ok(sent) => format!("ok({sent})"),
        Err(error) => format!("err({error})"),
    };
    log.append_line(
        "send",
        &[
            ("chunk", chunk_index.to_string()),
            ("items", summary),
            ("resend_all", resend_all.to_string()),
            ("result", outcome),
        ],
    );
}

#[cfg(test)]
mod tests {
    use super::record_send;
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
            status: deliver::Status::Changed,
        };
        record_send(&log, 0, &[&capture], false, &Ok(1));
        let text = std::fs::read_to_string(&log.path).unwrap();
        assert!(text.contains("event=send"));
        assert!(text.contains("items=one:changed"));
        assert!(text.contains("result=ok(1)"));
    }
}
