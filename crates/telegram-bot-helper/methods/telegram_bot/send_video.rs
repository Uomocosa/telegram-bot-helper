use crate::ApiResponse;
use crate::Caption;
use crate::Error;
use crate::TelegramBot;

pub fn send_video(bot: &TelegramBot, bytes: &[u8], caption: &Caption) -> Result<String, Error> {
    if bytes.is_empty() {
        return Err(Error::EmptyPayload { op: "send_video" });
    }
    let token = bot.token.as_str();
    let chat_id = bot.chat_id.as_str();
    let client = reqwest::blocking::Client::new();
    let url = format!("https://api.telegram.org/bot{token}/sendVideo");
    let form = reqwest::blocking::multipart::Form::new()
        .text("chat_id", chat_id.to_owned())
        .text("caption", caption.truncated())
        .part(
            "video",
            reqwest::blocking::multipart::Part::bytes(bytes.to_vec())
                .file_name("clip.mp4")
                .mime_str("video/mp4")
                .unwrap_or_else(|_| {
                    reqwest::blocking::multipart::Part::bytes(bytes.to_vec()).file_name("clip.mp4")
                }),
        );
    let response = client
        .post(&url)
        .multipart(form)
        .send()
        .map_err(|err| Error::Request {
            op: "sendVideo",
            source: err,
        })?;
    let status = response.status();
    let status_text = status.to_string();
    let body = response.text().unwrap_or_default();
    if !status.is_success() {
        let snippet: String = body.chars().take(500).collect();
        return Err(Error::Rejected {
            op: "sendVideo",
            status: status_text,
            snippet,
        });
    }
    ApiResponse {
        op: "sendVideo",
        status: status_text,
        body,
    }
    .message_id()
}

#[cfg(test)]
mod tests {
    use super::send_video;
    use crate::Caption;
    use crate::TelegramBot;

    #[test]
    fn test_usage() {
        let bot = TelegramBot {
            token: "token".to_string(),
            chat_id: "chat".to_string(),
        };
        assert!(send_video(&bot, &[], &Caption::from("caption")).is_err());
        let _ = send_video;
    }
}
