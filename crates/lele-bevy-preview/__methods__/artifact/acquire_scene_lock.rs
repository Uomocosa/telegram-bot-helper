use std::path::Path;

use crate::Error;
use crate::deliver;

pub fn acquire_scene_lock(out_dir: &Path, scene: &str) -> Result<std::fs::File, Error> {
    let locks = deliver::Artifact::locks_dir(out_dir);
    std::fs::create_dir_all(&locks).map_err(Error::Io)?;
    let lock_path = deliver::Artifact::lock_path(out_dir, scene);
    let lock_file = std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(&lock_path)
        .map_err(Error::Io)?;
    lock_file.lock().map_err(Error::Io)?;
    Ok(lock_file)
}

#[cfg(test)]
mod tests {
    use super::acquire_scene_lock;

    #[test]
    fn test_usage() {
        let _ = acquire_scene_lock;
    }
}
