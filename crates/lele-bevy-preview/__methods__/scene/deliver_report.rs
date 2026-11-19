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
    let log = deliver::DeliveryLog::from_out_dir_and_scene(&config.out_dir, &scene.name);
    let fps = match scene.timeline.as_ref() {
        Some(spec) => spec.fps,
        None if report.clip.is_some() => {
            let _ = std::fs::remove_dir_all(&staging_root);
            return Err(Error::Scene(format!(
                "scene `{}` has a clip but no timeline",
                scene.name
            )));
        }
        None => 0,
    };
    let mut fresh = match deliver::FreshDelivery::try_from_report_and_baseline(
        report,
        &config.out_dir,
        fps,
        &config.drift,
    ) {
        Ok(fresh) => fresh,
        Err(error) => {
            let _ = std::fs::remove_dir_all(&staging_root);
            return Err(error);
        }
    };
    for (index, capture) in fresh.captures.iter().enumerate() {
        let reason = fresh
            .reasons
            .get(index)
            .copied()
            .unwrap_or(deliver::Reason::NoBaseline);
        let diff = fresh.diffs.get(index).and_then(|found| found.as_ref());
        log.record_decision(capture, reason, diff);
    }
    if let (Some(clip), Some(reason)) = (fresh.clip.as_ref(), fresh.clip_reason) {
        log.record_decision(clip, reason, None);
    }
    for (slot, capture) in report.captures.iter_mut().zip(fresh.captures.iter()) {
        slot.status = capture.status;
    }
    if let (Some(slot), Some(capture)) = (report.clip.as_mut(), fresh.clip.as_ref()) {
        slot.status = capture.status;
    }
    let dry = std::env::var("LELE_PREVIEW_DRY_RUN").is_ok_and(|flag| flag == "1");
    if dry {
        log.record_skip("dry_run");
        let positions = fresh.all_positions();
        scene.stash_positions_to_unsent(&mut fresh, &positions, config, &staging_root);
        report.sync_paths_from_fresh(&fresh);
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
        log.record_skip("nothing_sendable");
        let positions = fresh.all_positions();
        scene.drop_positions_staged(&mut fresh, &positions, config, &staging_root);
        report.sync_paths_from_fresh(&fresh);
        let _ = std::fs::remove_dir_all(&staging_root);
        return Ok(0);
    }
    let sent = match scene.send_fresh_delivery_and_record(&mut fresh, config, &staging_root, &log) {
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
    use crate::Report;
    use crate::deliver;
    use crate::testing::config_with;
    use crate::testing::recording_send;
    use crate::testing::scene_with;
    use crate::testing::staging_capture;

    use super::deliver_report;

    #[test]
    fn test_usage() {
        let dir = tempfile::tempdir().unwrap();
        let staged = staging_capture(dir.path(), "ui", "one", 9, 9, 9, deliver::Status::Changed);
        let mut report = Report {
            captures: vec![staged],
            clip: None,
        };
        let sent = deliver_report(
            &scene_with("ui", &[], None),
            &mut report,
            &config_with(dir.path(), recording_send),
        )
        .unwrap();
        assert_eq!(sent, 1);
    }
}
