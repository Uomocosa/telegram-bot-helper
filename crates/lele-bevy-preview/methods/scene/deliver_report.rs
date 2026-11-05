use crate::Error;
use crate::deliver;
use crate::preview;
use crate::report::Report;
use crate::scene;

pub fn deliver_report(
    scene: &scene::Scene,
    report: &mut Report,
    config: &preview::Config,
) -> Result<usize, Error> {
    let staging_root = deliver::Artifact::staging_root(&config.out_dir, &scene.name);
    let _held = deliver::Artifact::acquire_scene_lock(&config.out_dir, &scene.name)?;
    let fps = match scene.timeline.as_ref() {
        Some(spec) => spec.fps,
        None if report.clip.is_some() => {
            for capture in report.captures.iter().chain(report.clip.iter()) {
                let entry = deliver::PreviewDecisionLog::from_capture_old_hash_baseline_and_outcome(
                    capture, "", "invalid", "failed",
                );
                let _ = entry.append(&config.out_dir);
            }
            let _ = std::fs::remove_dir_all(&staging_root);
            return Err(Error::Scene(format!(
                "scene `{}` has a clip but no timeline",
                scene.name
            )));
        }
        None => 0,
    };
    let mut fresh =
        match deliver::FreshDelivery::try_from_report_and_baseline(report, &config.out_dir, fps) {
            Ok(fresh) => fresh,
            Err(error) => {
                for capture in report.captures.iter().chain(report.clip.iter()) {
                    let entry =
                        deliver::PreviewDecisionLog::from_capture_old_hash_baseline_and_outcome(
                            capture, "", "invalid", "failed",
                        );
                    let _ = entry.append(&config.out_dir);
                }
                let _ = std::fs::remove_dir_all(&staging_root);
                return Err(error);
            }
        };
    for (slot, capture) in report.captures.iter_mut().zip(fresh.captures.iter()) {
        slot.status = capture.status;
    }
    if let (Some(slot), Some(capture)) = (report.clip.as_mut(), fresh.clip.as_ref()) {
        slot.status = capture.status;
    }
    let dry = std::env::var("LELE_PREVIEW_DRY_RUN").is_ok_and(|flag| flag == "1");
    if dry {
        let positions = fresh.all_positions();
        scene.stash_positions_to_unsent(&mut fresh, &positions, config, &staging_root);
        report.sync_paths_from_fresh(&fresh);
        let all: Vec<&deliver::Capture> = fresh.captures.iter().chain(fresh.clip.iter()).collect();
        let _ = fresh.log_captures_with_outcome(&all, "dry_run", &config.out_dir);
        let _ = std::fs::remove_dir_all(&staging_root);
        return Ok(0);
    }
    let sendable = fresh
        .captures
        .iter()
        .chain(fresh.clip.iter())
        .any(deliver::Capture::is_sendable);
    let resend = deliver::Capture::is_resend_all_requested();
    if !resend && !sendable {
        let positions = fresh.all_positions();
        scene.drop_positions_staged(&mut fresh, &positions, config, &staging_root);
        report.sync_paths_from_fresh(&fresh);
        let all: Vec<&deliver::Capture> = fresh.captures.iter().chain(fresh.clip.iter()).collect();
        let _ = fresh.log_captures_with_outcome(&all, "skipped_same", &config.out_dir);
        let _ = std::fs::remove_dir_all(&staging_root);
        return Ok(0);
    }
    let sent = match scene.send_fresh_delivery_and_record(&mut fresh, config, &staging_root) {
        Ok(sent) => sent,
        Err(error) => {
            report.sync_paths_from_fresh(&fresh);
            let _ = std::fs::remove_dir_all(&staging_root);
            return Err(error);
        }
    };
    report.sync_paths_from_fresh(&fresh);
    let _ = std::fs::remove_dir_all(&staging_root);
    Ok(sent)
}

#[cfg(test)]
mod tests {
    use super::deliver_report;

    #[test]
    fn test_usage() {
        let _ = deliver_report;
    }
}
