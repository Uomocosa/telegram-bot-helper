use crate::deliver;

#[must_use]
pub fn from_capture_old_hash_cache_and_outcome(
    capture: &deliver::Capture,
    old_hash: &str,
    cache: &str,
    outcome: &str,
) -> deliver::PreviewDecisionLog {
    let time = deliver::PreviewDecisionLog::current_time();
    let run = deliver::PreviewDecisionLog::current_run();
    let driver = deliver::PreviewDecisionLog::current_driver();
    let pid = std::process::id();
    deliver::PreviewDecisionLog {
        time,
        run,
        pid,
        test_scene: capture.artifact.scene.clone(),
        key: capture.artifact.fingerprint.clone(),
        status: capture.status_word().to_string(),
        old_hash: old_hash.to_string(),
        new_hash: capture.artifact.pixel_hash.clone(),
        cache: cache.to_string(),
        driver,
        outcome: outcome.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::from_capture_old_hash_cache_and_outcome;

    #[test]
    fn test_usage() {
        let _ = from_capture_old_hash_cache_and_outcome;
    }
}
