use serde_json::Value;

use crate::TelegramBot;

#[must_use]
pub fn latest_message<'u>(bot: &TelegramBot, updates: &'u [Value]) -> Option<&'u Value> {
    let wanted: i64 = bot.chat_id.parse().ok()?;
    updates.iter().rev().find_map(|update| {
        let message = ["message", "edited_message", "channel_post"]
            .into_iter()
            .find_map(|key| update.get(key))?;
        let id = message
            .get("chat")
            .and_then(|chat| chat.get("id"))
            .and_then(Value::as_i64)?;
        (id == wanted).then_some(message)
    })
}

#[cfg(test)]
mod tests {
    use serde_json::Value;

    use super::latest_message;
    use crate::TelegramBot;

    #[test]
    fn test_usage() {
        let bot = TelegramBot {
            token: "token".to_string(),
            chat_id: "111".to_string(),
        };
        let updates = serde_json::json!([
            {"update_id": 1, "message": {"message_id": 1, "chat": {"id": 111}, "text": "old"}},
            {"update_id": 2, "message": {"message_id": 2, "chat": {"id": 222}, "text": "other"}},
            {"update_id": 3, "message": {"message_id": 3, "chat": {"id": 111}, "text": "new"}},
        ]);
        let updates: &Vec<Value> = updates.as_array().unwrap();
        let latest = latest_message(&bot, updates).unwrap();
        assert_eq!(latest.get("text").and_then(Value::as_str), Some("new"));
        let other = TelegramBot {
            token: "token".to_string(),
            chat_id: "999".to_string(),
        };
        assert!(latest_message(&other, updates).is_none());
        let not_a_number = TelegramBot {
            token: "token".to_string(),
            chat_id: "not-a-number".to_string(),
        };
        assert!(latest_message(&not_a_number, updates).is_none());
        assert!(latest_message(&bot, &[]).is_none());
    }
}
