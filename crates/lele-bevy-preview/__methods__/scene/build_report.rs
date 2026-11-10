use crate::Error;
use crate::deliver;
use crate::preview;
use crate::report::Report;
use crate::scene;

pub fn build_report(scene: &scene::Scene, config: &preview::Config) -> Result<Report, Error> {
    scene.check()?;
    let staging_root = deliver::Artifact::staging_root(&config.out_dir, &scene.name);
    let _ = std::fs::remove_dir_all(&staging_root);
    std::fs::create_dir_all(&staging_root).map_err(Error::Io)?;
    let rendered = scene.render_states(config, &staging_root)?;
    let captures = deliver::Capture::collect(&scene.name, scene.kind, &rendered, &config.out_dir)?;
    let clip = scene.build_clip(config, &staging_root)?;
    Ok(Report { captures, clip })
}

#[cfg(test)]
mod tests {
    use super::build_report;

    #[test]
    fn test_usage() {
        let _ = build_report;
    }
}
