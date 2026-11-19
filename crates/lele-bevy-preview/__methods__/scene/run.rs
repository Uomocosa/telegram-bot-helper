use crate::Error;
use crate::preview;
use crate::report::Report;
use crate::scene;

pub fn run(scene: &scene::Scene, config: &preview::Config) -> Result<Report, Error> {
    let mut report = scene.build_report(config)?;
    scene.deliver_report(&mut report, config)?;
    scene::Scene::announce_artifacts(&report.captures, report.clip.as_ref());
    Ok(report)
}

#[cfg(test)]
mod tests {
    use crate::preview;
    use crate::testing::fast_config;
    use crate::testing::recording_send;
    use crate::testing::scene_with;

    use super::run;

    #[test]
    fn test_usage() {
        let dir = tempfile::tempdir().unwrap();
        let config = preview::Config {
            send_preview_group: recording_send,
            ..fast_config(dir.path())
        };
        let report = run(&scene_with("ui", &["one room"], None), &config).unwrap();
        assert_eq!(report.captures.len(), 1);
        assert!(report.captures[0].path.exists());
    }
}
