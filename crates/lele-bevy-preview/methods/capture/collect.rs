use std::path::Path;

use crate::Error;
use crate::deliver;
use crate::deliver::basic::enums::Media;
use crate::preview;
use crate::scene;

pub fn collect(
    scene: &str,
    kind: scene::Kind,
    rendered: &[preview::RenderedState],
    out_dir: &Path,
) -> Result<Vec<deliver::Capture>, Error> {
    let mut captures = Vec::new();
    for state in rendered {
        let artifact = deliver::Artifact {
            scene: scene.to_string(),
            label: state.label.clone(),
            fingerprint: deliver::Artifact::fingerprint_of(scene, &state.label),
            pixel_hash: state.pixel_hash.clone(),
            media: Media::Png,
            kind,
        };
        let status = artifact.status_in_cache(out_dir)?;
        captures.push(deliver::Capture {
            path: state.path.clone(),
            artifact,
            status,
        });
    }
    Ok(captures)
}

#[cfg(test)]
mod tests {
    use super::collect;

    #[test]
    fn test_usage() {
        let _ = collect;
    }
}
