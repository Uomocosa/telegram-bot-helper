use atomic_delegate_macros::atomic_delegates;

use crate::Error;
use crate::deliver;
use crate::report::Report;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FreshDelivery {
    pub captures: Vec<deliver::Capture>,
    pub reasons: Vec<deliver::Reason>,
    pub clip: Option<deliver::Capture>,
    pub clip_reason: Option<deliver::Reason>,
}

#[atomic_delegates]
impl FreshDelivery {
    pub(crate) fn try_from_report_and_baseline(
        report: &Report,
        out_dir: &std::path::Path,
        fps: u32,
        drift: &deliver::DriftTolerance,
    ) -> Result<Self, Error> {
    }
}

#[atomic_delegates]
impl FreshDelivery {
    pub(crate) fn all_positions(&self) -> Vec<(usize, bool)> {}
    pub(crate) fn chunk_refs<'a>(&'a self, chunk: &[(usize, bool)]) -> Vec<&'a deliver::Capture> {}
    pub(crate) fn chunk_is_sendable(&self, chunk: &[(usize, bool)]) -> bool {}
}

#[cfg(test)]
mod tests {
    use super::FreshDelivery;

    #[test]
    fn test_usage() {
        let fresh = FreshDelivery {
            captures: Vec::new(),
            reasons: Vec::new(),
            clip: None,
            clip_reason: None,
        };
        assert_eq!(fresh.captures.len(), 0);
        assert!(fresh.clip.is_none());
    }
}
