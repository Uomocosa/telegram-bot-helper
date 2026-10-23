use std::ffi::OsStr;
use std::path::Path;

use crate::__basic__;
use crate::Error;
use crate::deliver;

pub fn send_one(capture: &deliver::Capture, caption: &str) -> Result<bool, Error> {
    let Some(bot) = telegram_bot_helper::TelegramBot::from_env() else {
        println!("{}", __basic__::constants::NO_CREDENTIALS);
        return Ok(false);
    };
    let caption = telegram_bot_helper::Caption::from(caption);
    let uploaded = if is_video(&capture.path) {
        bot.send_video_file(&capture.path, &caption)
    } else {
        bot.send_photo_file(&capture.path, &caption)
    };
    uploaded.map_err(|error| Error::Telegram(error.to_string()))?;
    Ok(true)
}

// needed helper: extension-based media choice, so the zed task needs only a path
fn is_video(path: &Path) -> bool {
    path.extension().and_then(OsStr::to_str) == Some("mp4")
}

#[cfg(test)]
mod tests {
    use super::send_one;

    #[test]
    fn test_usage() {
        let _ = send_one;
    }
}
