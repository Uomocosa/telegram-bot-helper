use crate::Error;
use crate::deliver;

pub fn send_group(items: &[&deliver::Capture], caption: &str) -> Result<usize, Error> {
    let [only] = items else {
        return deliver::Capture::send_album(items, caption);
    };
    let sent = only.send_one(caption)?;
    Ok(usize::from(sent))
}

#[cfg(test)]
mod tests {
    use crate::deliver;
    use crate::testing::capture;

    use super::send_group;

    #[test]
    fn test_usage() {
        unsafe { std::env::set_var("TELEGRAM_BOT_TOKEN", "") };
        unsafe { std::env::set_var("TELEGRAM_CHAT_ID", "") };
        let item = capture(
            "shot.png",
            "rest",
            deliver::Media::Png,
            deliver::Status::New,
        );
        let sent = send_group(&[&item], "caption").unwrap();
        unsafe { std::env::remove_var("TELEGRAM_BOT_TOKEN") };
        unsafe { std::env::remove_var("TELEGRAM_CHAT_ID") };
        assert_eq!(sent, 0);
    }
}
