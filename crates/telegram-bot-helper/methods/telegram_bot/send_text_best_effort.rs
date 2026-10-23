use crate::TelegramBot;

pub fn send_text_best_effort(text: &str) {
    let Some(bot) = TelegramBot::from_env() else {
        eprintln!("telegram skipped (no creds): {text}");
        return;
    };
    if let Err(err) = bot.send_text(text) {
        eprintln!("telegram send failed: {err}");
    }
}

#[cfg(test)]
mod tests {
    use super::send_text_best_effort;

    #[test]
    fn test_usage() {
        send_text_best_effort("");
    }
}
