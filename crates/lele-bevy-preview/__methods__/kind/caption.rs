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

    #[test]
    fn test_usage() {
        let _ = caption;
    }
}
