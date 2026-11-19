use std::path::Path;

use crate::deliver;

pub fn solid_png(path: &Path, red: u8, green: u8, blue: u8) -> String {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).unwrap();
    }
    let image = image::RgbaImage::from_pixel(8, 8, image::Rgba([red, green, blue, 255]));
    image.save(path).unwrap();
    deliver::Artifact::hash_png_pixels(path).unwrap()
}

#[cfg(test)]
mod tests {
    use super::solid_png;
    use crate::deliver;

    #[test]
    fn test_usage() {
        let dir = tempfile::tempdir().unwrap();
        let hash = solid_png(&dir.path().join("solid.png"), 1, 2, 3);
        assert_eq!(hash.len(), deliver::HASH_CHARS);
    }
}
