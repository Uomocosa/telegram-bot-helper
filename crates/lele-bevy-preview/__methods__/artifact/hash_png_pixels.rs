use std::path::Path;

use crate::Error;
use crate::deliver;

pub fn hash_png_pixels(path: &Path) -> Result<String, Error> {
    let image = image::open(path).map_err(|source| Error::Image {
        path: path.to_path_buf(),
        source,
    })?;
    let rgba = image.to_rgba8();
    Ok(deliver::Artifact::hash_bytes(rgba.as_raw()))
}

#[cfg(test)]
mod tests {
    use super::hash_png_pixels;

    #[test]
    fn test_usage() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("solid.png");
        let solid = image::RgbaImage::from_pixel(4, 4, image::Rgba([10, 12, 16, 255]));
        solid.save(&path).unwrap();
        let first = hash_png_pixels(&path).unwrap();
        let second = hash_png_pixels(&path).unwrap();
        assert_eq!(first, second);
        assert_eq!(first.len(), crate::deliver::HASH_CHARS);
    }
}
