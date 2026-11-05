use std::path::Path;

use crate::Error;
use crate::deliver::basic::constants;

pub fn acquire_exclusive_lock(dir: &Path) -> Result<std::fs::File, Error> {
    std::fs::create_dir_all(dir).map_err(Error::Io)?;
    let lock_path = dir.join(constants::MANIFEST_LOCK_FILE);
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
    use super::acquire_exclusive_lock;

    #[test]
    fn test_usage() {
        let _ = acquire_exclusive_lock;
    }
}
