use crate::Error;
use crate::basic;
use crate::deliver;

pub fn send_one(capture: &deliver::Capture, caption: &str) -> Result<bool, Error> {
    let Some(bot) = telegram_bot_helper::TelegramBot::from_env() else {
        println!("{}", basic::constants::NO_CREDENTIALS);
        return Ok(false);
    };
    let caption = telegram_bot_helper::Caption::from(caption);
    let uploaded = if capture.is_mp4() {
        bot.send_video_file(&capture.path, &caption)
    } else {
        bot.send_photo_file(&capture.path, &caption)
    };
    uploaded.map_err(Error::Telegram)?;
    Ok(true)
}

#[cfg(test)]
mod tests {
    use super::send_one;

    #[test]
    fn test_usage() {
        let _ = send_one;
    }
}
