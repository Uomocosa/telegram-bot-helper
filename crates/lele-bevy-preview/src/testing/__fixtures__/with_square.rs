use crate::testing;

pub fn with_square(x_start: u32, y_start: u32) -> image::RgbaImage {
    let mut image = testing::canvas(24, 24);
    for y in y_start..y_start.saturating_add(10) {
        for x in x_start..x_start.saturating_add(10) {
            image.put_pixel(x, y, image::Rgba([250, 250, 250, 255]));
        }
    }
    image
}

#[cfg(test)]
mod tests {
    use super::with_square;
    use crate::testing::canvas;

    #[test]
    fn test_usage() {
        assert_ne!(with_square(5, 5), canvas(24, 24));
    }
}
