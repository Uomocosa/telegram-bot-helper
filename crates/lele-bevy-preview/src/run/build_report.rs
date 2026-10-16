use crate::Error;
use crate::deliver;
use crate::preview;
use crate::report::Report;
use crate::scene::Scene;
use crate::timeline;

pub fn build_report(scene: &Scene, config: &preview::Config) -> Result<Report, Error> {
    let previous = deliver::load_previous::load_previous(&config.out_dir)
        .map(|manifest| deliver::keep_on_disk::keep_on_disk(manifest, &config.out_dir));
    let rendered = preview::render_states::render_states(scene, config)?;
    let captures =
        deliver::collect::collect(&scene.name, scene.kind, &rendered, previous.as_ref())?;
    let clip = timeline::build_clip::build_clip(scene, config, previous.as_ref())?;
    Ok(Report { captures, clip })
}
// no test_usage necessary
