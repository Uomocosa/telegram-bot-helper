use crate::deliver;

#[must_use]
pub const fn status_word(capture: &deliver::Capture) -> &'static str {
    let status = capture.status;
    match status {
        deliver::Status::New => "new",
        deliver::Status::Changed => "changed",
        deliver::Status::Same => "same",
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::status_word;
    use crate::deliver;
    use crate::scene;

    fn artifact(scene: &str, label: &str, pixel_hash: &str) -> deliver::Artifact {
        deliver::Artifact {
            scene: String::from(scene),
            label: String::from(label),
            fingerprint: deliver::Artifact::fingerprint_of(scene, label),
            pixel_hash: String::from(pixel_hash),
            media: deliver::Media::Png,
            kind: scene::Kind::System,
        }
    }

    fn capture(status: deliver::Status) -> deliver::Capture {
        deliver::Capture {
            path: PathBuf::from("shot.png"),
            artifact: artifact("ui", "rest", "hash"),
            status,
        }
    }

    #[test]
    fn test_usage() {
        assert_eq!(status_word(&capture(deliver::Status::New)), "new");
        assert_eq!(status_word(&capture(deliver::Status::Changed)), "changed");
        assert_eq!(status_word(&capture(deliver::Status::Same)), "same");
    }
}
