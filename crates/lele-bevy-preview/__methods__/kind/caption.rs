use std::fmt::Write;

use crate::deliver;
use crate::scene;

#[must_use]
pub fn caption(kind: scene::Kind, scene_name: &str, items: &[&deliver::Capture]) -> String {
    let mut out = format!("{} {scene_name}", kind.tag());
    for (index, capture) in items.iter().enumerate() {
        let icon = match capture.artifact.media {
            deliver::Media::Png => "🖼",
            deliver::Media::Mp4 => "▶",
        };
        let _ = write!(
            out,
            "\n{}. {icon} {} · {}",
            index.saturating_add(1),
            capture.artifact.label,
            capture.status_word()
        );
    }
    out
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::caption;
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

    fn capture(label: &str, media: deliver::Media, status: deliver::Status) -> deliver::Capture {
        let mut item = artifact("ui", label, "hash");
        item.media = media;
        deliver::Capture {
            path: PathBuf::from("shot"),
            artifact: item,
            status,
        }
    }

    #[test]
    fn test_usage() {
        let rest = capture("rest", deliver::Media::Png, deliver::Status::New);
        let press = capture("press", deliver::Media::Mp4, deliver::Status::Changed);
        assert_eq!(
            caption(scene::Kind::Plugin, "ui", &[&rest, &press]),
            "[plugin] ui\n1. 🖼 rest · new\n2. ▶ press · changed"
        );
        assert_eq!(caption(scene::Kind::System, "ui", &[]), "[system] ui");
    }
}
