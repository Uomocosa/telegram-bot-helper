use std::path::Path;

pub fn dot(path: &Path, x: u32, y: u32) {
    let mut image = image::RgbaImage::from_pixel(10, 10, image::Rgba([255, 255, 255, 255]));
    image.put_pixel(x, y, image::Rgba([0, 0, 0, 255]));
    image.save(path).unwrap();
}

#[cfg(test)]
mod tests {
    use super::dot;

    #[test]
    fn test_usage() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("dot.png");
        dot(&path, 5, 5);
        assert!(path.exists());
    }
}
