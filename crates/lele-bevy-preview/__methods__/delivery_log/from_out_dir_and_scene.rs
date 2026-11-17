use std::path::Path;

use crate::basic;
use crate::deliver;

#[must_use]
pub fn from_out_dir_and_scene(out_dir: &Path, scene: &str) -> deliver::DeliveryLog {
    let pid = std::process::id();
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(0, |elapsed| elapsed.as_nanos());
    let seed = format!("{pid}-{nanos}");
    let hash = deliver::Artifact::hash_bytes(seed.as_bytes());
    let run_id: String = hash.chars().take(deliver::HASH_CHARS / 2).collect();
    deliver::DeliveryLog {
        path: out_dir
            .join(basic::constants::LOG_DIR)
            .join("deliveries.log"),
        run_id,
        scene: scene.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::from_out_dir_and_scene;

    #[test]
    fn test_usage() {
        let dir = tempfile::tempdir().unwrap();
        let first = from_out_dir_and_scene(dir.path(), "ui");
        let second = from_out_dir_and_scene(dir.path(), "ui");
        assert!(first.path.ends_with("_log/deliveries.log"));
        assert_eq!(first.run_id.len(), 8);
        assert_eq!(first.scene, "ui");
        assert_eq!(first.path, second.path);
    }
}
