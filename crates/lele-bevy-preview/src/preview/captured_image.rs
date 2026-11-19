use std::path::Path;

use atomic_delegate_macros::atomic_delegates;
use bevy::image::Image;
use derive_more::Deref;

use crate::Error;

#[derive(Deref)]
pub struct CapturedImage(pub Image);

#[rustfmt::skip]
impl CapturedImage {
    #[must_use]
    pub fn bytes(&self) -> &[u8] { self.data.as_deref().unwrap_or_default() }
}

#[atomic_delegates]
impl CapturedImage {
    #[must_use]
    pub fn distinct_colors(&self) -> usize {}
    pub fn ensure_visual(&self, out: &Path, min: usize) -> Result<(), Error> {}
}

#[cfg(test)]
mod tests {
    use crate::testing::captured;

    #[test]
    fn test_usage() {
        let image = captured(&[[1, 2, 3, 255], [1, 2, 3, 255]]);
        let colors = image.distinct_colors();
        assert_eq!(colors, 1);
        assert_eq!(image.bytes().len(), 8);
    }
}
