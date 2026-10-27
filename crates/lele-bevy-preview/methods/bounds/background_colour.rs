use std::collections::HashMap;

#[must_use]
pub fn background_colour(image: &image::RgbaImage) -> Option<image::Rgba<u8>> {
    let mut counts: HashMap<image::Rgba<u8>, usize> = HashMap::new();
    for pixel in image.pixels() {
        let count = counts.entry(*pixel).or_insert(0);
        *count = count.saturating_add(1);
    }
    counts
        .into_iter()
        .max_by_key(|(_, count)| *count)
        .map(|(pixel, _)| pixel)
}

#[cfg(test)]
mod tests {
    use super::background_colour;

    #[test]
    fn test_usage() {
        let white = image::Rgba([255, 255, 255, 255]);
        let red = image::Rgba([255, 0, 0, 255]);
        let mut image = image::RgbaImage::from_pixel(4, 4, white);
        image.put_pixel(0, 0, red);
        image.put_pixel(1, 0, red);
        assert_eq!(background_colour(&image), Some(white));
        assert_eq!(background_colour(&image::RgbaImage::new(0, 0)), None);
    }
}
