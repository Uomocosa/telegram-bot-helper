use std::path::Path;

use crate::Caption;
use crate::Error;
use crate::TelegramBot;

pub fn send_photo_file(bot: &TelegramBot, path: &Path, caption: &Caption) -> Result<String, Error> {
    let display = path.display().to_string();
    let bytes = std::fs::read(path).map_err(|source| Error::Read {
        path: display.clone(),
        source,
    })?;
    if bytes.len() < 1024 {
        return Err(Error::TooSmall {
            kind: "preview",
            path: display,
            bytes: bytes.len(),
        });
    }
    if !is_png(&bytes) {
        return Err(Error::NotPng { path: display });
    }
    bot.send_photo(&bytes, caption)
}

// needed helper: png magic check, so a non-png file names itself
fn is_png(bytes: &[u8]) -> bool {
    const MAGIC: [u8; 8] = [137, 80, 78, 71, 13, 10, 26, 10];
    bytes.len() >= MAGIC.len() && bytes.starts_with(&MAGIC)
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::send_photo_file;
    use crate::Caption;
    use crate::TelegramBot;

    #[test]
    fn test_usage() {
        let bot = TelegramBot {
            token: "token".to_string(),
            chat_id: "chat".to_string(),
        };
        let caption = Caption::from("c");
        assert!(send_photo_file(&bot, Path::new("/nonexistent-preview.png"), &caption).is_err());
        let dir = std::env::temp_dir().join("telegram-bot-send-photo-file-test");
        let _ = std::fs::create_dir_all(&dir);
        let empty = dir.join("empty.png");
        let _ = std::fs::write(&empty, b"");
        assert!(send_photo_file(&bot, &empty, &caption).is_err());
        let _ = std::fs::remove_file(&empty);
    }
}
