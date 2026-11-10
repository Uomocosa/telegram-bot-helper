use serde_json::{Value, json};

use crate::MediaGroup;
use crate::MediaKind;

#[must_use]
pub fn manifest(group: &MediaGroup) -> String {
    let entries: Vec<Value> = group
        .items
        .iter()
        .enumerate()
        .map(|(index, item)| {
            let kind = match item.kind {
                MediaKind::Photo => "photo",
                MediaKind::Video => "video",
            };
            let mut entry = serde_json::Map::new();
            entry.insert("type".to_string(), json!(kind));
            entry.insert("media".to_string(), json!(format!("attach://file{index}")));
            if index == 0 {
                entry.insert(
                    "caption".to_string(),
                    Value::String(group.caption.truncated()),
                );
            }
            Value::Object(entry)
        })
        .collect();
    Value::Array(entries).to_string()
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::manifest;
    use crate::Caption;
    use crate::MediaGroup;
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
        let parsed: Value = serde_json::from_str(&manifest(&group)).unwrap_or(Value::Null);
        let entries = parsed.as_array().cloned().unwrap_or_default();
        assert_eq!(entries.len(), 2);
        assert_eq!(entries[0]["caption"], "hello");
        assert_eq!(entries[1]["media"], "attach://file1");
    }
}
