use atomic_delegate_macros::atomic_delegates;

use crate::Error;
use crate::deliver;
use crate::report::Report;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FreshDelivery {
    pub captures: Vec<deliver::Capture>,
    pub clip: Option<deliver::Capture>,
}

#[atomic_delegates]
impl FreshDelivery {
    pub(crate) fn try_from_report_and_baseline(
        report: &Report,
        out_dir: &std::path::Path,
        fps: u32,
    ) -> Result<Self, Error> {
    }
}

#[atomic_delegates]
impl FreshDelivery {
    pub(crate) fn all_positions(&self) -> Vec<(usize, bool)> {}
}

#[rustfmt::skip]
impl FreshDelivery {

    pub(crate) fn chunk_refs<'a>(&'a self, chunk: &[(usize, bool)]) -> Vec<&'a deliver::Capture> {
        chunk
            .iter()
            .filter_map(|(position, is_clip)| {
                if *is_clip {
                    self.clip.as_ref()
                } else {
                    self.captures.get(*position)
                }
            })
            .collect()
    }

    pub(crate) fn chunk_is_sendable(&self, chunk: &[(usize, bool)]) -> bool {
        chunk.iter().any(|(position, is_clip)| {
            if *is_clip {
                self.clip
                    .as_ref()
                    .is_some_and(deliver::Capture::is_sendable)
            } else {
                self.captures
                    .get(*position)
                    .is_some_and(deliver::Capture::is_sendable)
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::FreshDelivery;

    #[test]
    fn test_usage() {
        let fresh = FreshDelivery {
            captures: Vec::new(),
            clip: None,
        };
        assert_eq!(fresh.captures.len(), 0);
        assert!(fresh.clip.is_none());
    }
}
