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
    use super::caption;
    use crate::deliver;
    use crate::scene;
    use crate::testing::capture;

    #[test]
    fn test_usage() {
        let rest = capture("shot", "rest", deliver::Media::Png, deliver::Status::New);
        let press = capture(
            "shot",
            "press",
            deliver::Media::Mp4,
            deliver::Status::Changed,
        );
        assert_eq!(
            caption(scene::Kind::Plugin, "ui", &[&rest, &press]),
            "[plugin] ui\n1. 🖼 rest · new\n2. ▶ press · changed"
        );
        assert_eq!(caption(scene::Kind::System, "ui", &[]), "[system] ui");
    }
}
