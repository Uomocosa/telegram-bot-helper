use crate::Error;
use crate::deliver;
use crate::preview;
use crate::scene;

pub fn render_states(
    scene: &scene::Scene,
    config: &preview::Config,
) -> Result<Vec<preview::RenderedState>, Error> {
    scene.check()?;
    let mut rendered = Vec::new();
    for state in &scene.states {
        let out = config.out_dir.join(format!(
            "{}__{}.png",
            scene.name,
            deliver::Artifact::slug(&state.label)
        ));
        let pixel_hash = scene.render_state_in_fresh_app(config, state, &out)?;
        rendered.push(preview::RenderedState {
            path: out,
            label: state.label.clone(),
            pixel_hash,
        });
    }
    Ok(rendered)
}

#[cfg(test)]
mod tests {
    use super::render_states;

    #[test]
    fn test_usage() {
        let _ = render_states;
    }
}
