use std::path::Path;

use crate::Error;
use crate::deliver;
use crate::deliver::basic::enums::Media;

pub fn send_group(items: &[&deliver::Capture], caption: &str) -> Result<usize, Error> {
    let [only] = items else {
        return send_album(items, caption);
    };
    let sent = only.send_one(caption)?;
    Ok(usize::from(sent))
}

// needed helper: albums need credentials too, and absent ones leave artifacts on disk only
fn send_album(items: &[&deliver::Capture], caption: &str) -> Result<usize, Error> {
    let Some(bot) = telegram_bot_helper::TelegramBot::from_env() else {
        println!("{}", crate::basic::constants::NO_CREDENTIALS);
        return Ok(0);
    };
    let media: Vec<telegram_bot_helper::MediaItem> = items
        .iter()
        .map(|capture| telegram_bot_helper::MediaItem {
            kind: media_kind(capture.artifact.media),
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

const fn media_kind(media: Media) -> telegram_bot_helper::MediaKind {
    match media {
        Media::Png => telegram_bot_helper::MediaKind::Photo,
        Media::Mp4 => telegram_bot_helper::MediaKind::Video,
    }
}

#[cfg(test)]
mod tests {
    use super::send_group;

    #[test]
    fn test_usage() {
        let _ = send_group;
    }
}
