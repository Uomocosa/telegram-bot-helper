use crate::deliver;

#[must_use]
pub const fn is_sendable(capture: &deliver::Capture) -> bool {
    let status = capture.status;
    matches!(status, deliver::Status::New | deliver::Status::Changed)
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::is_sendable;
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
        assert!(is_sendable(&capture(deliver::Status::New)));
        assert!(is_sendable(&capture(deliver::Status::Changed)));
        assert!(!is_sendable(&capture(deliver::Status::Same)));
    }
}
