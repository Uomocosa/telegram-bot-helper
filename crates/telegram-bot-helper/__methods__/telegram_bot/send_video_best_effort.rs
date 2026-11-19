use std::path::Path;

use crate::Caption;
use crate::TelegramBot;

pub fn send_video_best_effort(clip: &Path, caption: &Caption) {
    let Some(bot) = TelegramBot::from_env() else {
        eprintln!("telegram skipped (no creds): clip={}", clip.display());
        return;
    };
    match bot.send_video_file(clip, caption) {
        Ok(message) => println!(
            "telegram video sent: message_id={message} clip={}",
            clip.display()
        ),
        Err(err) => eprintln!(
            "telegram send failed: {} clip={}",
            err.with_causes(),
            clip.display()
        ),
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::send_video_best_effort;
    use crate::Caption;

    #[test]
    fn sending_a_missing_clip_without_credentials_does_not_panic() {
        send_video_best_effort(Path::new("/nonexistent-clip.mp4"), &Caption::from("smoke"));
    }
}

// no test_usage necessary
