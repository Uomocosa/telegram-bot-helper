use crate::TelegramBot;

#[must_use]
pub fn from_env() -> Option<TelegramBot> {
    let _ = dotenvy::dotenv();
    let token = std::env::var("TELEGRAM_BOT_TOKEN")
        .or_else(|_| std::env::var("TG_BOT_TOKEN"))
        .ok()?;
    let chat_id = std::env::var("TELEGRAM_CHAT_ID")
        .or_else(|_| std::env::var("TG_CHAT_ID"))
        .ok()?;
    if token.is_empty() || chat_id.is_empty() {
        return None;
    }
    Some(TelegramBot { token, chat_id })
}

#[cfg(test)]
mod tests {
    use super::from_env;

    #[test]
    fn test_usage() {
        unsafe { std::env::set_var("TELEGRAM_BOT_TOKEN", "123:test") };
        unsafe { std::env::set_var("TELEGRAM_CHAT_ID", "42") };
        let bot = from_env();
        assert!(bot.is_some());
        let bot = bot.unwrap();
        assert_eq!(bot.token, "123:test");
        assert_eq!(bot.chat_id, "42");
        unsafe { std::env::remove_var("TELEGRAM_BOT_TOKEN") };
        unsafe { std::env::remove_var("TELEGRAM_CHAT_ID") };
    }
}
