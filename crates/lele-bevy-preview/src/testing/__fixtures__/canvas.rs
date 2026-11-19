pub fn canvas(width: u32, height: u32) -> image::RgbaImage {
    image::RgbaImage::from_pixel(width, height, image::Rgba([10, 12, 16, 255]))
}

#[cfg(test)]
mod tests {
    use super::canvas;

    #[test]
    fn test_usage() {
        assert_eq!(canvas(24, 16).dimensions(), (24, 16));
    }
}
