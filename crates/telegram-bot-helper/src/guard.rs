use crate::TelegramBot;

pub struct Guard {
    pub name: String,
    pub start: std::time::Instant,
}

impl From<&str> for Guard {
    fn from(name: &str) -> Self {
        Self {
            name: name.to_string(),
            start: std::time::Instant::now(),
        }
    }
}

impl Drop for Guard {
    fn drop(&mut self) {
        if !TelegramBot::is_status_reporting_enabled() {
            return;
        }
        let status = if std::thread::panicking() {
            "FAILED"
        } else {
            "PASSED"
        };
        let text = format!("{} {} in {:?}", status, self.name, self.start.elapsed());
        TelegramBot::send_text_best_effort(&text);
    }
}

#[cfg(test)]
mod tests {
    use super::Guard;

    #[test]
    fn test_usage() {
        let guard = Guard::from("guard_start_smoke");
        assert_eq!(guard.name, "guard_start_smoke");
    }
}
