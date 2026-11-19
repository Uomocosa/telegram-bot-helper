use crate::testing;

pub fn with_horizontal_border(y: u32) -> image::RgbaImage {
    let mut image = testing::canvas(24, 24);
    for x in 0..24 {
        image.put_pixel(x, y, image::Rgba([250, 250, 250, 255]));
    }
    image
}

#[cfg(test)]
mod tests {
    use super::with_horizontal_border;
    use crate::testing::canvas;

    #[test]
    fn test_usage() {
        assert_ne!(with_horizontal_border(10), canvas(24, 24));
    }
}
