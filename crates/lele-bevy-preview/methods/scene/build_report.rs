use crate::Error;
use crate::deliver;
use crate::preview;
use crate::report::Report;
use crate::scene;

pub fn build_report(scene: &scene::Scene, config: &preview::Config) -> Result<Report, Error> {
    let previous = deliver::Manifest::load(&config.out_dir)
        .map(|manifest| manifest.keep_on_disk(&config.out_dir));
    let rendered = scene.render_states(config)?;
    let captures =
        deliver::Capture::collect(&scene.name, scene.kind, &rendered, previous.as_ref())?;
    let clip = scene.build_clip(config, previous.as_ref())?;
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
