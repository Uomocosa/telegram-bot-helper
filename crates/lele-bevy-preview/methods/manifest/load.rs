use std::path::Path;

use crate::deliver;
use crate::deliver::basic::constants;

#[must_use]
pub fn load(dir: &Path) -> Option<deliver::Manifest> {
    let text = std::fs::read_to_string(dir.join(constants::MANIFEST_FILE)).ok()?;
    serde_json::from_str(&text).ok()
}

#[cfg(test)]
mod tests {
    use super::load;

    #[test]
    fn test_usage() {
        let _ = load;
    }
}
