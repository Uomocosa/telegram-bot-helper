use crate::Error;
use crate::deliver;
use crate::preview;
use crate::report::Report;
use crate::scene;

pub fn deliver_report(
    scene: &scene::Scene,
    report: &Report,
    config: &preview::Config,
    crate_name: &str,
) -> Result<usize, Error> {
    let _held = deliver::SentPreviewHash::acquire_scene_lock(&config.out_dir, &scene.name)?;
    let fresh = match deliver::FreshDelivery::try_from_report_and_cache(report, &config.out_dir) {
        Ok(fresh) => fresh,
        Err(error) => {
            for capture in report.captures.iter().chain(report.clip.iter()) {
                let entry = deliver::PreviewDecisionLog::from_capture_old_hash_cache_and_outcome(
                    capture, "", "invalid", "failed",
                );
                let _ = entry.append(&config.out_dir);
            }
            return Err(error);
        }
    };
    let dry = std::env::var("LELE_PREVIEW_DRY_RUN").is_ok_and(|flag| flag == "1");
    if dry {
        let all: Vec<&deliver::Capture> = fresh.captures.iter().chain(fresh.clip.iter()).collect();
        let _ = fresh.log_captures_with_outcome(&all, "dry_run", &config.out_dir);
        return Ok(0);
    }
    let sendable = fresh
        .captures
        .iter()
        .chain(fresh.clip.iter())
        .any(deliver::Capture::is_sendable);
    let resend = deliver::Capture::is_resend_all_requested();
    if !resend && !sendable {
        let all: Vec<&deliver::Capture> = fresh.captures.iter().chain(fresh.clip.iter()).collect();
        let _ = fresh.log_captures_with_outcome(&all, "skipped_same", &config.out_dir);
        return Ok(0);
    }
    scene.send_fresh_delivery_and_record(&fresh, config, crate_name)
}

#[cfg(test)]
mod tests {
    use super::deliver_report;

    #[test]
    fn test_usage() {
        let _ = deliver_report;
    }
}
