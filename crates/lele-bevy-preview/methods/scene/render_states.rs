use std::path::Path;

use crate::Error;
use crate::PreviewApp;
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
        let pixel_hash = render_one(scene, config, state, &out)?;
        rendered.push(preview::RenderedState {
            path: out,
            label: state.label.clone(),
            pixel_hash,
        });
    }
    Ok(rendered)
}

// needed helper: every state needs its own App, because a spawn from state N must not leak into N+1
fn render_one(
    scene: &scene::Scene,
    config: &preview::Config,
    state: &scene::State,
    out: &Path,
) -> Result<String, Error> {
    let mut app = PreviewApp::from_scene(scene, config);
    app.warmup();
    (state.apply)(app.app.world_mut());
    app.app.update();
    let frame = app.settled(out, config.max_capture_frames, config.min_distinct_colors)?;
    Ok(deliver::Artifact::hash_bytes(frame.bytes()))
}

#[cfg(test)]
mod tests {
    use super::render_states;

    #[test]
    fn test_usage() {
        let _ = render_states;
    }
}
