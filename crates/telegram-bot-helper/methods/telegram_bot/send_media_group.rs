use reqwest::blocking::multipart::{Form, Part};

use crate::ApiResponse;
use crate::Error;
use crate::MediaGroup;
use crate::MediaKind;
use crate::TelegramBot;

const MIN_ITEMS: usize = 2;
const MAX_ITEMS: usize = 10;

pub fn send_media_group(bot: &TelegramBot, group: &MediaGroup) -> Result<Vec<String>, Error> {
    if !(MIN_ITEMS..=MAX_ITEMS).contains(&group.items.len()) {
        return Err(Error::BadAlbumSize {
            count: group.items.len(),
        });
    }
    let token = bot.token.as_str();
    let mut form = Form::new().text("chat_id", bot.chat_id.clone());
    for (index, item) in group.items.iter().enumerate() {
        let bytes = TelegramBot::read_album_item(item)?;
        let (file_name, mime) = match item.kind {
            MediaKind::Photo => ("preview.png", "image/png"),
            MediaKind::Video => ("clip.mp4", "video/mp4"),
        };
        let part = Part::bytes(bytes).file_name(file_name);
        form = form.part(
            format!("file{index}"),
            part.mime_str(mime).map_err(|err| Error::Request {
                op: "sendMediaGroup",
                source: err,
            })?,
        );
    }
    form = form.text("media", group.manifest());
    let url = format!("https://api.telegram.org/bot{token}/sendMediaGroup");
    let response = reqwest::blocking::Client::new()
        .post(&url)
        .multipart(form)
        .send()
        .map_err(|err| Error::Request {
            op: "sendMediaGroup",
            source: err,
        })?;
    let status = response.status();
    let status_text = status.to_string();
    let body = response.text().unwrap_or_default();
    if !status.is_success() {
        let snippet: String = body.chars().take(500).collect();
        return Err(Error::Rejected {
            op: "sendMediaGroup",
            status: status_text,
            snippet,
        });
    }
    ApiResponse {
        op: "sendMediaGroup",
        status: status_text,
        body,
    }
    .message_ids()
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::send_media_group;
    use crate::Caption;
    use crate::MediaGroup;
    use crate::MediaItem;
    use crate::MediaKind;
    use crate::TelegramBot;

    #[test]
    fn test_usage() {
        let bot = TelegramBot {
            token: "token".to_string(),
            chat_id: "chat".to_string(),
        };
        let one = [MediaItem {
            kind: MediaKind::Photo,
            path: Path::new("/nonexistent-a.png"),
        }];
        let too_small = MediaGroup {
            items: &one,
            caption: Caption::from("c"),
        };
        assert!(send_media_group(&bot, &too_small).is_err());
        let two = [one[0], one[0]];
        let pair = MediaGroup {
            items: &two,
            caption: Caption::from("c"),
        };
        let err = send_media_group(&bot, &pair).unwrap_err();
        assert!(err.to_string().contains("nonexistent-a.png"));
    }
}
