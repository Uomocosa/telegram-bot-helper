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
    use super::CapturedImage;

    #[test]
    fn test_usage() {
        let _ = CapturedImage::ensure_visual;
    }
}
