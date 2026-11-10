use serde_json::Value;

use crate::ApiResponse;
use crate::Error;
use crate::TelegramBot;

pub fn get_updates(
    bot: &TelegramBot,
    offset: Option<i64>,
    timeout_secs: u64,
) -> Result<Vec<Value>, Error> {
    let token = bot.token.as_str();
    let client = reqwest::blocking::Client::new();
    let url = format!("https://api.telegram.org/bot{token}/getUpdates");
    let payload = offset.map_or_else(
        || serde_json::json!({ "limit": 100, "timeout": timeout_secs }),
        |offset| serde_json::json!({ "limit": 100, "timeout": timeout_secs, "offset": offset }),
    );
    let response = client
        .post(&url)
        .json(&payload)
        .send()
        .map_err(|err| Error::Request {
            op: "getUpdates",
            source: err,
        })?;
    let status = response.status();
    let status_text = status.to_string();
    let body = response.text().unwrap_or_default();
    if !status.is_success() {
        let snippet: String = body.chars().take(500).collect();
        return Err(Error::Rejected {
            op: "getUpdates",
            status: status_text,
            snippet,
        });
    }
    ApiResponse {
        op: "getUpdates",
        status: status_text,
        body,
    }
    .result_value()
    .map(|result| result.as_array().cloned().unwrap_or_default())
}

#[cfg(test)]
mod tests {
    use super::get_updates;

    #[test]
    fn test_usage() {
        let _ = get_updates;
    }
}
