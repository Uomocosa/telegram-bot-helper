use atomic_delegate_macros::atomic_delegates;

use crate::deliver;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DriftTolerance {
    pub channel_delta: u8,
    pub min_pixels: u32,
    pub min_density: f32,
    pub cluster_gap: u32,
    pub max_shift: i32,
}

impl Default for DriftTolerance {
    fn default() -> Self {
        Self {
            channel_delta: 2,
            min_pixels: 8,
            min_density: 0.25,
            cluster_gap: 2,
            max_shift: 1,
        }
    }
}

#[rustfmt::skip]
impl DriftTolerance {
    #[must_use]
    pub fn status_against_images(&self, baseline: &image::RgbaImage, new: &image::RgbaImage) -> deliver::Status { self.reason_against_images(baseline, new).status() }
}

#[atomic_delegates]
impl DriftTolerance {
    #[must_use]
    pub fn reason_against_images(
        &self,
        baseline: &image::RgbaImage,
        new: &image::RgbaImage,
    ) -> deliver::Reason {
    }
    #[must_use]
    pub fn pixel_diff(
        &self,
        baseline: &image::RgbaImage,
        new: &image::RgbaImage,
    ) -> deliver::PixelDiff {
    }
}

#[atomic_delegates]
impl DriftTolerance {
    pub(crate) fn changed_mask(
        &self,
        baseline: &image::RgbaImage,
        new: &image::RgbaImage,
    ) -> Vec<bool> {
    }
    pub(crate) fn changed_clusters(
        &self,
        mask: &[bool],
        width: u32,
        height: u32,
    ) -> Vec<Vec<(u32, u32)>> {
    }
    pub(crate) fn is_cluster_drift(
        &self,
        cluster: &[(u32, u32)],
        baseline: &image::RgbaImage,
        new: &image::RgbaImage,
    ) -> bool {
    }
    pub(crate) fn does_shift_explain_cluster(
        &self,
        cluster: &[(u32, u32)],
        baseline: &image::RgbaImage,
        new: &image::RgbaImage,
    ) -> bool {
    }
    pub(crate) fn pixel_index(width: u32, x: u32, y: u32) -> usize {}
}

#[cfg(test)]
mod tests {
    use super::DriftTolerance;

    #[test]
    fn test_usage() {
        let tolerance = DriftTolerance::default();
        assert_eq!(tolerance.channel_delta, 2);
        assert_eq!(tolerance.min_pixels, 8);
        assert_eq!(tolerance.min_density, 0.25);
        assert_eq!(tolerance.cluster_gap, 2);
        assert_eq!(tolerance.max_shift, 1);
    }
}
