use std::path::PathBuf;

use crate::Report;
use crate::deliver;
use crate::testing;

pub fn report(path: PathBuf, pixel_hash: &str) -> Report {
    Report {
        captures: vec![deliver::Capture {
            path,
            artifact: testing::artifact("ui", "rest", pixel_hash),
            status: deliver::Status::Changed,
        }],
        clip: None,
    }
}

#[cfg(test)]
mod tests {
    use super::report;
    use std::path::PathBuf;

    #[test]
    fn test_usage() {
        assert_eq!(report(PathBuf::from("shot.png"), "hash").captures.len(), 1);
    }
}
