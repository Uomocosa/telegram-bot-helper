use std::path::Path;

use crate::Caption;
use crate::Error;
use crate::TelegramBot;

pub fn send_video_file(bot: &TelegramBot, path: &Path, caption: &Caption) -> Result<String, Error> {
    let display = path.display().to_string();
    let bytes = std::fs::read(path).map_err(|source| Error::Read {
        path: display.clone(),
        source,
    })?;
    if bytes.len() < 1024 {
        return Err(Error::TooSmall {
            kind: "clip",
            path: display,
            bytes: bytes.len(),
        });
    }
    if !bytes.windows(4).any(|window| window == b"ftyp") {
        return Err(Error::NotMp4 { path: display });
    }
    bot.send_video(&bytes, caption)
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::send_video_file;
    use crate::Caption;
    use crate::TelegramBot;

    #[test]
    fn test_usage() {
        let bot = TelegramBot {
            token: "token".to_string(),
            chat_id: "chat".to_string(),
        };
        let caption = Caption::from("c");
        assert!(send_video_file(&bot, Path::new("/nonexistent-clip.mp4"), &caption).is_err());
        let dir = std::env::temp_dir().join("telegram-bot-send-video-file-test");
        let _ = std::fs::create_dir_all(&dir);
        let empty = dir.join("empty.mp4");
        let _ = std::fs::write(&empty, b"");
        assert!(send_video_file(&bot, &empty, &caption).is_err());
        let _ = std::fs::remove_file(&empty);
    }
}
