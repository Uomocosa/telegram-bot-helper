use crate::deliver;
use crate::scene;

pub fn artifact(scene: &str, label: &str, pixel_hash: &str) -> deliver::Artifact {
    deliver::Artifact {
        scene: String::from(scene),
        label: String::from(label),
        fingerprint: deliver::Artifact::fingerprint_of(scene, label),
        pixel_hash: String::from(pixel_hash),
        media: deliver::Media::Png,
        kind: scene::Kind::System,
    }
}

#[cfg(test)]
mod tests {
    use super::artifact;

    #[test]
    fn test_usage() {
        assert_eq!(artifact("ui", "rest", "hash").fingerprint, "ui|rest");
    }
}
