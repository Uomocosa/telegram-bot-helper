use crate::Error;
use crate::deliver;
use crate::preview;
use crate::report::Report;
use crate::scene::Scene;

pub mod build_report;

pub fn run(scene: &Scene, config: &preview::Config, crate_name: &str) -> Result<Report, Error> {
    let report = build_report::build_report(scene, config)?;
    let sent = deliver::send_changed::send_changed(scene, &report.captures, report.clip.as_ref())?;
    let (recorded, pending): (Vec<&deliver::Capture>, Vec<&deliver::Capture>) = report
        .captures
        .iter()
        .chain(report.clip.iter())
        .partition(|capture| capture.status == deliver::Status::Same || sent > 0);
    deliver::update_manifest::update_manifest(&config.out_dir, crate_name, &recorded, &pending)?;
    announce(&report.captures, report.clip.as_ref());
    Ok(report)
}

// needed helper: one line per artifact so a human reviewing test output sees what changed
fn announce(captures: &[deliver::Capture], clip: Option<&deliver::Capture>) {
    for capture in captures {
        println!(
            "PREVIEW_ARTIFACT={} status={} label={}",
            capture.path.display(),
            label(capture.status),
            capture.artifact.label
        );
    }
    if let Some(clip) = clip {
        println!(
            "PREVIEW_ARTIFACT={} status={} label={}",
            clip.path.display(),
            label(clip.status),
            clip.artifact.label
        );
    }
    println!(
        "PREVIEW_MANIFEST={}",
        deliver::basic::constants::MANIFEST_FILE
    );
}

const fn label(status: deliver::Status) -> &'static str {
    match status {
        deliver::Status::New => "new",
        deliver::Status::Changed => "changed",
        deliver::Status::Same => "same",
    }
}
// no test_usage necessary
