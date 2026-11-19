use crate::testing;

pub fn with_bordered_square(colour: image::Rgba<u8>) -> image::RgbaImage {
    let mut image = testing::canvas(24, 24);
    for y in 0..24 {
        for x in 0..24 {
            if !(3..21).contains(&x) || !(3..21).contains(&y) {
                image.put_pixel(x, y, colour);
            }
        }
    }
    image
}

#[cfg(test)]
mod tests {
    use super::with_bordered_square;
    use crate::testing::canvas;

    #[test]
    fn test_usage() {
        let red = image::Rgba([200, 40, 40, 255]);
        assert_ne!(with_bordered_square(red), canvas(24, 24));
    }
}
