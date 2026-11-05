use std::path::Path;

use crate::deliver;

pub fn log_captures_with_outcome(
    fresh: &deliver::FreshDelivery,
    captures: &[&deliver::Capture],
    outcome: &str,
    out_dir: &Path,
) -> usize {
    let mut appended: usize = 0;
    for capture in captures {
        let old = fresh
            .old_hashes
            .get(&capture.artifact.fingerprint)
            .map_or("", String::as_str);
        let entry = deliver::PreviewDecisionLog::from_capture_old_hash_baseline_and_outcome(
            capture,
            old,
            &fresh.baseline_state,
            outcome,
        );
        if entry.append(out_dir).is_ok() {
            appended = appended.saturating_add(1);
        }
    }
    appended
}

#[cfg(test)]
mod tests {
    use super::log_captures_with_outcome;

    #[test]
    fn test_usage() {
        let _ = log_captures_with_outcome;
    }
}
