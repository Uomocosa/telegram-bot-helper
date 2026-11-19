use crate::TelegramBot;

pub fn bot() -> TelegramBot {
    TelegramBot {
        token: String::from("123:test"),
        chat_id: String::from("1"),
    }
}

#[cfg(test)]
mod tests {
    use super::bot;

    #[test]
    fn test_usage() {
        assert_eq!(bot().token, "123:test");
    }
}
