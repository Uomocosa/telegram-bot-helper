use std::collections::HashMap;

use atomic_delegate_macros::atomic_delegates;

use crate::deliver;
use crate::report::Report;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FreshDelivery {
    pub captures: Vec<deliver::Capture>,
    pub clip: Option<deliver::Capture>,
    pub old_hashes: HashMap<String, String>,
    pub manifest_state: String,
}

#[atomic_delegates]
impl FreshDelivery {
    #[must_use]
    pub(crate) fn from_report_and_manifest(
        report: &Report,
        filtered: Option<&deliver::Manifest>,
    ) -> Self {
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
            manifest_state: String::from("missing"),
        };
        assert_eq!(fresh.captures.len(), 0);
        assert!(fresh.manifest_state.contains("missing"));
    }
}
