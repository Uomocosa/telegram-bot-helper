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
    use crate::deliver;
    use crate::testing::capture;

    use super::send_one;

    #[test]
    fn test_usage() {
        unsafe { std::env::set_var("TELEGRAM_BOT_TOKEN", "") };
        unsafe { std::env::set_var("TELEGRAM_CHAT_ID", "") };
        let item = capture(
            "shot.png",
            "rest",
            deliver::Media::Png,
            deliver::Status::New,
        );
        let sent = send_one(&item, "caption").unwrap();
        unsafe { std::env::remove_var("TELEGRAM_BOT_TOKEN") };
        unsafe { std::env::remove_var("TELEGRAM_CHAT_ID") };
        assert!(!sent);
    }
}
