use std::path::Path;

use atomic_delegate_macros::atomic_delegates;
use serde_json::Value;

use crate::Caption;
use crate::Error;
use crate::MediaGroup;
use crate::MediaItem;

pub struct TelegramBot {
    pub token: String,
    pub chat_id: String,
}

#[atomic_delegates]
impl TelegramBot {
    #[must_use]
    pub fn from_env() -> Option<Self> {}
    #[must_use]
    pub fn is_status_reporting_enabled() -> bool {}
    pub fn send_text(&self, text: &str) -> Result<String, Error> {}
    pub fn send_photo(&self, bytes: &[u8], caption: &Caption) -> Result<String, Error> {}
    pub fn send_photo_file(&self, path: &Path, caption: &Caption) -> Result<String, Error> {}
    pub fn send_video(&self, bytes: &[u8], caption: &Caption) -> Result<String, Error> {}
    pub fn send_video_file(&self, path: &Path, caption: &Caption) -> Result<String, Error> {}
    pub fn send_media_group(&self, group: &MediaGroup) -> Result<Vec<String>, Error> {}
    pub fn get_updates(&self, offset: Option<i64>, timeout_secs: u64) -> Result<Vec<Value>, Error> {
    }
    #[must_use]
    pub fn latest_message<'u>(&self, updates: &'u [Value]) -> Option<&'u Value> {}
    pub fn send_text_best_effort(text: &str) {}
    pub fn send_video_best_effort(clip: &Path, caption: &Caption) {}
    pub(crate) fn read_album_item(item: &MediaItem) -> Result<Vec<u8>, Error> {}
}

#[rustfmt::skip]
impl TelegramBot {
    pub(crate) fn is_png(bytes: &[u8]) -> bool { bytes.starts_with(&[137, 80, 78, 71, 13, 10, 26, 10]) }
    pub(crate) fn has_ftyp(bytes: &[u8]) -> bool { bytes.windows(4).any(|window| window == b"ftyp") }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::TelegramBot;
    use crate::Caption;
    use crate::MediaGroup;
    use crate::MediaItem;
    use crate::MediaKind;
    use serde_json::Value;

    fn bot() -> TelegramBot {
        TelegramBot {
            token: "token".to_string(),
            chat_id: "chat".to_string(),
        }
    }

    #[test]
    fn test_usage() {
        let bot = bot();
        assert!(bot.send_text("").is_err());
        let caption = Caption::from("caption");
        assert!(bot.send_photo(&[], &caption).is_err());
        assert!(bot.send_video(&[], &caption).is_err());
        assert!(
            bot.send_photo_file(Path::new("/nonexistent-preview.png"), &caption)
                .is_err()
        );
        let item = MediaItem {
            kind: MediaKind::Photo,
            path: Path::new("/nonexistent-a.png"),
        };
        let items = [item];
        let group = MediaGroup {
            items: &items,
            caption,
        };
        assert!(bot.send_media_group(&group).is_err());
        let updates: Vec<Value> = Vec::new();
        assert!(bot.latest_message(&updates).is_none());
        assert!(TelegramBot::is_png(&[137, 80, 78, 71, 13, 10, 26, 10, 0]));
        assert!(!TelegramBot::is_png(b"GIF89a"));
        assert!(TelegramBot::has_ftyp(b"\0\0\0\x18ftypmp42"));
        assert!(!TelegramBot::has_ftyp(b"moov"));
    }

    #[test]
    fn test_usage_environment_is_optional() {
        let _ = TelegramBot::from_env();
        let _ = TelegramBot::is_status_reporting_enabled();
    }
}
