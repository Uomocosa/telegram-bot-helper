use crate::load_creds;
use crate::send_text;
use crate::status_enabled;

pub struct Guard {
    pub name: String,
    pub start: std::time::Instant,
}

impl Guard {
    #[must_use]
    pub fn start(name: &str) -> Self {
        Self {
            name: name.to_string(),
            start: std::time::Instant::now(),
        }
    }
}

impl Drop for Guard {
    fn drop(&mut self) {
        let status = if std::thread::panicking() {
            "FAILED"
        } else {
            "PASSED"
        };
        let text = format!("{} {} in {:?}", status, self.name, self.start.elapsed());
        if !status_enabled() {
            return;
        }
        let Some(creds) = load_creds() else {
            eprintln!("telegram skipped (no creds): {text}");
            return;
        };
        if let Err(err) = send_text(&creds, &text) {
            eprintln!("telegram send failed: {err}");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Guard;

    #[test]
    fn test_usage() {
        let guard = Guard::start("guard_start_smoke");
        assert_eq!(guard.name, "guard_start_smoke");
    }
}
