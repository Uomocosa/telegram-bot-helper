use std::path::Path;

use crate::Error;
use crate::deliver;
use crate::deliver::basic::constants;

pub fn write_merged_atomically(dir: &Path, manifest: &deliver::Manifest) -> Result<(), Error> {
    let json = serde_json::to_string_pretty(manifest)?;
    let tmp = dir.join(format!("{}.tmp", constants::MANIFEST_FILE));
    std::fs::write(&tmp, &json).map_err(Error::Io)?;
    std::fs::rename(&tmp, dir.join(constants::MANIFEST_FILE)).map_err(Error::Io)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::write_merged_atomically;

    #[test]
    fn test_usage() {
        let _ = write_merged_atomically;
    }
}
