use atomic_delegate_macros::atomic_delegates;

use crate::Error;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Bounds {
    pub min_x: u32,
    pub min_y: u32,
    pub max_x: u32,
    pub max_y: u32,
    pub width: u32,
    pub height: u32,
}

#[atomic_delegates]
impl Bounds {
    pub fn of_image(path: &std::path::Path) -> Result<Option<Bounds>, Error> {}
    pub(crate) fn background_colour(image: &image::RgbaImage) -> Option<image::Rgba<u8>> {}
}

#[cfg(test)]
mod tests {
    use super::Bounds;

    #[test]
    fn test_usage() {
        let bounds = Bounds {
            min_x: 1,
            min_y: 2,
            max_x: 3,
            max_y: 4,
            width: 10,
            height: 10,
        };
        assert_eq!(bounds.max_x - bounds.min_x, 2);
        let _ = Bounds::of_image;
    }
}
