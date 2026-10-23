use atomic_delegate_macros::atomic_delegates;

use crate::Caption;
use crate::MediaItem;

pub struct MediaGroup<'a> {
    pub items: &'a [MediaItem<'a>],
    pub caption: Caption<'a>,
}

#[atomic_delegates]
impl MediaGroup<'_> {
    #[must_use]
    pub fn manifest(&self) -> String {}
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::MediaGroup;
    use crate::Caption;
    use crate::MediaItem;
    use crate::MediaKind;
    use serde_json::Value;

    #[test]
    fn test_usage() {
        let items = [
            MediaItem {
                kind: MediaKind::Photo,
                path: Path::new("a.png"),
            },
            MediaItem {
                kind: MediaKind::Video,
                path: Path::new("b.mp4"),
            },
        ];
        let group = MediaGroup {
            items: &items,
            caption: Caption::from("hello"),
        };
        let parsed: Value = serde_json::from_str(&group.manifest()).unwrap_or(Value::Null);
        let entries = parsed.as_array().cloned().unwrap_or_default();
        assert_eq!(entries.len(), 2);
        assert_eq!(entries[0]["type"], "photo");
        assert_eq!(entries[0]["media"], "attach://file0");
        assert_eq!(entries[0]["caption"], "hello");
        assert_eq!(entries[1]["type"], "video");
        assert_eq!(entries[1]["media"], "attach://file1");
        assert!(entries[1].get("caption").is_none());
    }
}
