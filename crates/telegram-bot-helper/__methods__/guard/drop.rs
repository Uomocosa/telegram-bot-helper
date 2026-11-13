use crate::Guard;
use crate::TelegramBot;

pub fn drop(guard: &Guard) {
    if !TelegramBot::is_status_reporting_enabled() {
        return;
    }
    let status = if std::thread::panicking() {
        "FAILED"
    } else {
        "PASSED"
    };
    let text = format!("{} {} in {:?}", status, guard.name, guard.start.elapsed());
    TelegramBot::send_text_best_effort(&text);
}

#[cfg(test)]
mod tests {
    use super::drop;
    use crate::Guard;

    #[test]
    fn test_usage() {
        let guard = Guard::from("drop_smoke");
        drop(&guard);
        std::mem::forget(guard);
    }
}
