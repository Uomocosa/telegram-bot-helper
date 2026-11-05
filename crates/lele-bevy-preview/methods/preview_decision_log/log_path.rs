use std::path::Path;

use crate::deliver;

#[must_use]
pub fn log_path(out_dir: &Path) -> std::path::PathBuf {
    let name = deliver::PreviewDecisionLog::file_name();
    out_dir.join(name)
}

#[cfg(test)]
mod tests {
    use super::log_path;

    #[test]
    fn test_usage() {
        let _ = log_path;
    }
}
