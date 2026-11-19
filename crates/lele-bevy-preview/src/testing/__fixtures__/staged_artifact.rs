use std::path::Path;

use crate::deliver;
use crate::testing;

pub fn staged_artifact(new: &image::RgbaImage, dir: &Path) -> deliver::Artifact {
    let new_path = dir.join("staged.png");
    new.save(&new_path).unwrap();
    testing::artifact(
        "ui",
        "rest",
        &deliver::Artifact::hash_png_pixels(&new_path).unwrap(),
    )
}

#[cfg(test)]
mod tests {
    use super::staged_artifact;
    use crate::deliver;
    use crate::testing::canvas;

    #[test]
    fn test_usage() {
        let dir = tempfile::tempdir().unwrap();
        let item = staged_artifact(&canvas(8, 8), dir.path());
        assert_eq!(item.pixel_hash.len(), deliver::HASH_CHARS);
    }
}
