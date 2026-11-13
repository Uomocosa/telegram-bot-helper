use std::path::Path;

use crate::Error;
use crate::deliver;
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
            media: deliver::Media::Png,
            kind,
        };
        let status = artifact.status_against_baseline(out_dir)?;
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
    use std::path::PathBuf;

    use super::collect;
    use crate::deliver;
    use crate::preview;
    use crate::scene;

    #[test]
    fn test_usage() {
        let dir = tempfile::tempdir().unwrap();
        let rendered = vec![preview::RenderedState {
            path: PathBuf::from("staging/rest.png"),
            label: String::from("rest"),
            pixel_hash: String::from("unused"),
        }];
        let fresh = collect("ui", scene::Kind::System, &rendered, dir.path()).unwrap();
        assert_eq!(fresh.len(), 1);
        assert_eq!(fresh[0].status, deliver::Status::New);
        assert_eq!(fresh[0].artifact.fingerprint, "ui|rest");
        assert_eq!(fresh[0].path, PathBuf::from("staging/rest.png"));

        let baseline = {
            let delivered = fresh[0].artifact.delivered_path(dir.path());
            image::RgbaImage::from_pixel(4, 4, image::Rgba([7, 7, 7, 255]))
                .save(&delivered)
                .unwrap();
            deliver::Artifact::hash_png_pixels(&delivered).unwrap()
        };
        let matching = vec![preview::RenderedState {
            path: PathBuf::from("staging/rest.png"),
            label: String::from("rest"),
            pixel_hash: baseline,
        }];
        let same = collect("ui", scene::Kind::System, &matching, dir.path()).unwrap();
        assert_eq!(same[0].status, deliver::Status::Same);
    }
}
