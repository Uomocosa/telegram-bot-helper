use std::path::Path;

use crate::Error;
use crate::deliver;

pub fn send_album(items: &[&deliver::Capture], caption: &str) -> Result<usize, Error> {
    let Some(bot) = telegram_bot_helper::TelegramBot::from_env() else {
        println!("{}", crate::basic::constants::NO_CREDENTIALS);
        return Ok(0);
    };
    let media: Vec<telegram_bot_helper::MediaItem> = items
        .iter()
        .map(|capture| telegram_bot_helper::MediaItem {
            kind: capture.telegram_media_kind(),
            path: Path::new(&capture.path),
        })
        .collect();
    let group = telegram_bot_helper::MediaGroup {
        items: &media,
        caption: telegram_bot_helper::Caption::from(caption),
    };
    bot.send_media_group(&group).map_err(Error::Telegram)?;
    Ok(items.len())
}

#[cfg(test)]
mod tests {
    use super::send_album;

    #[test]
    fn test_usage() {
        let _ = send_album;
    }
}
