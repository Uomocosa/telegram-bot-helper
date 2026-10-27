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
    scene::Scene::announce_artifacts(&report.captures, report.clip.as_ref());
    Ok(report)
}

#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    fn test_usage() {
        let _ = run;
    }
}
