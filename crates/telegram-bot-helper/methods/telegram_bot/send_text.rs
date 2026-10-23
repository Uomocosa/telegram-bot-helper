use crate::ApiResponse;
use crate::Error;
use crate::TelegramBot;

pub fn send_text(bot: &TelegramBot, text: &str) -> Result<String, Error> {
    if text.is_empty() {
        return Err(Error::EmptyPayload { op: "send_text" });
    }
    let token = bot.token.as_str();
    let chat_id = bot.chat_id.as_str();
    let client = reqwest::blocking::Client::new();
    let url = format!("https://api.telegram.org/bot{token}/sendMessage");
    let response = client
        .post(&url)
        .json(&serde_json::json!({
            "chat_id": chat_id,
            "text": text
        }))
        .send()
        .map_err(|err| Error::Request {
            op: "sendMessage",
            source: err,
        })?;
    let status = response.status();
    let status_text = status.to_string();
    let body = response.text().unwrap_or_default();
    if !status.is_success() {
        let snippet: String = body.chars().take(500).collect();
        return Err(Error::Rejected {
            op: "sendMessage",
            status: status_text,
            snippet,
        });
    }
    ApiResponse {
        op: "sendMessage",
        status: status_text,
        body,
    }
    .message_id()
}

#[cfg(test)]
mod tests {
    use super::send_text;
    use crate::TelegramBot;

    #[test]
    fn test_usage() {
        let bot = TelegramBot {
            token: "token".to_string(),
            chat_id: "chat".to_string(),
        };
        assert!(send_text(&bot, "").is_err());
        let _ = send_text;
    }
}
