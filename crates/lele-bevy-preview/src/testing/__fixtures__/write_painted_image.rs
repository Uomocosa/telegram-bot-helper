use std::path::Path;

pub fn write_painted_image(path: &Path, paint: impl Fn(u32, u32) -> image::Rgba<u8>) {
    let mut image = image::RgbaImage::new(10, 10);
    for (x, y, pixel) in image.enumerate_pixels_mut() {
        *pixel = paint(x, y);
    }
    image.save(path).unwrap();
}

#[cfg(test)]
mod tests {
    use super::write_painted_image;

    #[test]
    fn test_usage() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("painted.png");
        write_painted_image(&path, |_, _| image::Rgba([255, 255, 255, 255]));
        assert!(path.exists());
    }
}
