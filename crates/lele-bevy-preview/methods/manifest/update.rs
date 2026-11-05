use std::collections::BTreeMap;
use std::path::Path;

use crate::Error;
use crate::deliver;

pub fn update(
    dir: &Path,
    crate_name: &str,
    recorded: &[&deliver::Capture],
    pending: &[&deliver::Capture],
) -> Result<(), Error> {
    let _held = deliver::Manifest::acquire_exclusive_lock(dir)?;
    let empty = deliver::Manifest {
        crate_name: crate_name.to_string(),
        artifacts: BTreeMap::new(),
    };
    let loaded = deliver::Manifest::load(dir)?;
    let mut merged = loaded.unwrap_or(empty);
    merged.crate_name = crate_name.to_string();
    for capture in pending {
        merged.artifacts.remove(&capture.artifact.fingerprint);
    }
    for capture in recorded {
        merged.artifacts.insert(
            capture.artifact.fingerprint.clone(),
            capture.artifact.clone(),
        );
    }
    deliver::Manifest::write_merged_atomically(dir, &merged)
}

#[cfg(test)]
mod tests {
    use super::update;

    #[test]
    fn test_usage() {
        let _ = update;
    }
}
