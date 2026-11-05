use std::collections::HashMap;

use atomic_delegate_macros::atomic_delegates;

use crate::Error;
use crate::deliver;
use crate::report::Report;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FreshDelivery {
    pub captures: Vec<deliver::Capture>,
    pub clip: Option<deliver::Capture>,
    pub old_hashes: HashMap<String, String>,
    pub cache_state: String,
}

#[atomic_delegates]
impl FreshDelivery {
    pub(crate) fn try_from_report_and_cache(
        report: &Report,
        out_dir: &std::path::Path,
    ) -> Result<Self, Error> {
    }
    pub(crate) fn log_captures_with_outcome(
        &self,
        captures: &[&deliver::Capture],
        outcome: &str,
        out_dir: &std::path::Path,
    ) -> usize {
    }
}

#[cfg(test)]
mod tests {
    use super::FreshDelivery;
    use std::collections::HashMap;

    #[test]
    fn test_usage() {
        let fresh = FreshDelivery {
            captures: Vec::new(),
            clip: None,
            old_hashes: HashMap::new(),
            cache_state: String::from("missing"),
        };
        assert_eq!(fresh.captures.len(), 0);
        assert!(fresh.cache_state.contains("missing"));
    }
}
