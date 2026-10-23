use crate::Error;
use crate::deliver;
use crate::preview;
use crate::report::Report;
use crate::scene;

pub fn run(
    scene: &scene::Scene,
    config: &preview::Config,
    crate_name: &str,
) -> Result<Report, Error> {
    let report = scene.build_report(config)?;
    let sent = scene.send_changed(&report.captures, report.clip.as_ref())?;
    let (recorded, pending): (Vec<&deliver::Capture>, Vec<&deliver::Capture>) = report
        .captures
        .iter()
        .chain(report.clip.iter())
        .partition(|capture| capture.status == deliver::Status::Same || sent > 0);
    deliver::Manifest::update(&config.out_dir, crate_name, &recorded, &pending)?;
    announce(&report.captures, report.clip.as_ref());
    Ok(report)
}

// needed helper: one line per artifact so a human reviewing test output sees what changed
fn announce(captures: &[deliver::Capture], clip: Option<&deliver::Capture>) {
    for capture in captures {
        println!(
            "PREVIEW_ARTIFACT={} status={} label={}",
            capture.path.display(),
            capture.status_word(),
            capture.artifact.label
        );
    }
    if let Some(clip) = clip {
        println!(
            "PREVIEW_ARTIFACT={} status={} label={}",
            clip.path.display(),
            clip.status_word(),
            clip.artifact.label
        );
    }
    println!(
        "PREVIEW_MANIFEST={}",
        deliver::basic::constants::MANIFEST_FILE
    );
}

#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    fn test_usage() {
        let _ = run;
    }
}
