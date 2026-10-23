use telegram_bot_helper::TelegramBot;

#[test]
#[ignore = "needs TELEGRAM_BOT_TOKEN/TELEGRAM_CHAT_ID; polls getUpdates (read-only)"]
fn live_read_updates() {
    let Some(bot) = TelegramBot::from_env() else {
        eprintln!("skipping live_read_updates: no telegram creds");
        return;
    };
    let updates = bot.get_updates(None, 0).expect("live getUpdates");
    eprintln!("live_read_updates: {} update(s) in window", updates.len());
    let Ok(expect) = std::env::var("TELEGRAM_EXPECT_TEXT") else {
        return;
    };
    if expect.is_empty() {
        return;
    }
    let latest = bot.latest_message(&updates);
    let content = latest
        .and_then(|message| message.get("text").or_else(|| message.get("caption")))
        .and_then(|value| value.as_str())
        .unwrap_or("");
    assert!(
        content.contains(&expect),
        "expected {expect:?} in latest message, got {content:?}"
    );
}
