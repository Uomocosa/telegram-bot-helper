use crate::Error;
use crate::deliver;
use crate::deliver::basic::enums::Media;
use crate::deliver::basic::structs::{Artifact, Capture, Manifest};
use crate::preview;
use crate::scene;

pub fn collect(
    scene: &str,
    kind: scene::Kind,
    rendered: &[preview::RenderedState],
    previous: Option<&Manifest>,
) -> Result<Vec<Capture>, Error> {
    let mut captures = Vec::new();
    for state in rendered {
        let artifact = Artifact {
            scene: scene.to_string(),
            label: state.label.clone(),
            fingerprint: deliver::fingerprint_of::fingerprint_of(scene, &state.label),
            pixel_hash: state.pixel_hash.clone(),
            media: Media::Png,
            kind,
            unsent: false,
        };
        let status = deliver::state_status::state_status(previous, &artifact);
        captures.push(Capture {
            path: state.path.clone(),
            artifact,
            status,
        });
    }
    Ok(captures)
}
// no test_usage necessary
