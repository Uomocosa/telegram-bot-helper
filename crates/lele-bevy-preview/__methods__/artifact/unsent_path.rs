use std::path::Path;

use crate::basic;
use crate::deliver;

#[must_use]
pub fn unsent_path(artifact: &deliver::Artifact, out_dir: &Path) -> std::path::PathBuf {
    out_dir
        .join(basic::constants::UNSENT_DIR)
        .join(artifact.file_name())
}

#[cfg(test)]
mod tests {
    use super::unsent_path;
    use crate::basic;
    use crate::deliver;
    use crate::scene;
    use std::path::Path;

    #[test]
    fn test_usage() {
        let artifact = deliver::Artifact {
            scene: String::from("ui"),
            label: String::from("one room"),
            fingerprint: String::new(),
            pixel_hash: String::new(),
            media: deliver::Media::Png,
            kind: scene::Kind::System,
        };
        let out_dir = Path::new("out");
        assert_eq!(
            unsent_path(&artifact, out_dir),
            out_dir
                .join(basic::constants::UNSENT_DIR)
                .join("ui__one_room.png")
        );
    }
}
