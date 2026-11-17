use crate::deliver;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Reason {
    NoBaseline,
    PixelHashEqual,
    DriftWithinTolerance,
    ExactModeDiffers,
    SizeDiffers,
    ClustersExceedDrift,
    NoBaselineFrames,
    ClipHashEqual,
    ClipHashDiffers,
}

impl Reason {
    #[must_use]
    pub const fn status(self) -> deliver::Status {
        match self {
            Self::NoBaseline | Self::NoBaselineFrames => deliver::Status::New,
            Self::PixelHashEqual | Self::DriftWithinTolerance | Self::ClipHashEqual => {
                deliver::Status::Same
            }
            Self::ExactModeDiffers
            | Self::SizeDiffers
            | Self::ClustersExceedDrift
            | Self::ClipHashDiffers => deliver::Status::Changed,
        }
    }
    #[must_use]
    pub const fn word(self) -> &'static str {
        match self {
            Self::NoBaseline => "no_baseline",
            Self::PixelHashEqual => "pixel_hash_equal",
            Self::DriftWithinTolerance => "drift_within_tolerance",
            Self::ExactModeDiffers => "exact_mode_differs",
            Self::SizeDiffers => "size_differs",
            Self::ClustersExceedDrift => "clusters_exceed_drift",
            Self::NoBaselineFrames => "no_baseline_frames",
            Self::ClipHashEqual => "clip_hash_equal",
            Self::ClipHashDiffers => "clip_hash_differs",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Reason;
    use crate::deliver;

    #[test]
    fn test_usage() {
        assert_eq!(Reason::NoBaseline.status(), deliver::Status::New);
        assert_eq!(Reason::NoBaseline.word(), "no_baseline");
        assert_eq!(Reason::ClipHashDiffers.word(), "clip_hash_differs");
    }

    #[test]
    fn test_reason_status_mapping() {
        assert_eq!(Reason::NoBaseline.status(), deliver::Status::New);
        assert_eq!(Reason::NoBaselineFrames.status(), deliver::Status::New);
        assert_eq!(Reason::PixelHashEqual.status(), deliver::Status::Same);
        assert_eq!(Reason::DriftWithinTolerance.status(), deliver::Status::Same);
        assert_eq!(Reason::ClipHashEqual.status(), deliver::Status::Same);
        assert_eq!(Reason::ExactModeDiffers.status(), deliver::Status::Changed);
        assert_eq!(Reason::SizeDiffers.status(), deliver::Status::Changed);
        assert_eq!(
            Reason::ClustersExceedDrift.status(),
            deliver::Status::Changed
        );
        assert_eq!(Reason::ClipHashDiffers.status(), deliver::Status::Changed);
        assert_eq!(Reason::NoBaseline.word(), "no_baseline");
        assert_eq!(Reason::PixelHashEqual.word(), "pixel_hash_equal");
        assert_eq!(
            Reason::DriftWithinTolerance.word(),
            "drift_within_tolerance"
        );
        assert_eq!(Reason::ExactModeDiffers.word(), "exact_mode_differs");
        assert_eq!(Reason::SizeDiffers.word(), "size_differs");
        assert_eq!(Reason::ClustersExceedDrift.word(), "clusters_exceed_drift");
        assert_eq!(Reason::NoBaselineFrames.word(), "no_baseline_frames");
        assert_eq!(Reason::ClipHashEqual.word(), "clip_hash_equal");
        assert_eq!(Reason::ClipHashDiffers.word(), "clip_hash_differs");
    }
}
