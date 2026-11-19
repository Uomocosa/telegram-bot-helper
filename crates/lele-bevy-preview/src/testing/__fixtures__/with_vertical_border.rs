use crate::testing;

pub fn with_vertical_border(x: u32) -> image::RgbaImage {
    let mut image = testing::canvas(24, 24);
    for y in 0..24 {
        image.put_pixel(x, y, image::Rgba([250, 250, 250, 255]));
    }
    image
}

#[cfg(test)]
mod tests {
    use super::with_vertical_border;
    use crate::testing::canvas;

    #[test]
    fn test_usage() {
        assert_ne!(with_vertical_border(10), canvas(24, 24));
    }
}
