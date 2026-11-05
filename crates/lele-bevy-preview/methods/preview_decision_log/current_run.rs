#[must_use]
pub fn current_run() -> String {
    let run = std::env::var("NEXTEST_RUN_ID").unwrap_or_default();
    if run.is_empty() {
        String::from("local")
    } else {
        run
    }
}

#[cfg(test)]
mod tests {
    use super::current_run;

    #[test]
    fn test_usage() {
        let _ = current_run();
    }
}
