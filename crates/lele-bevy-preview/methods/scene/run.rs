use crate::Error;
use crate::preview;
use crate::report::Report;
use crate::scene;

pub fn run(scene: &scene::Scene, config: &preview::Config) -> Result<Report, Error> {
    let report = scene.build_report(config)?;
    scene.deliver_report(&report, config)?;
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
