use std::path::Path;

use crate::Error;
use crate::deliver;
use crate::deliver::basic::constants;

pub fn load(dir: &Path) -> Result<Option<deliver::Manifest>, Error> {
    let path = dir.join(constants::MANIFEST_FILE);
    let text = match std::fs::read_to_string(&path) {
        Ok(text) => text,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Err(error) => {
            return Err(Error::ManifestUnreadable {
                path,
                source: error,
            });
        }
    };
    let manifest: deliver::Manifest =
        serde_json::from_str(&text).map_err(|source| Error::ManifestCorrupt {
            path: path.clone(),
            source,
        })?;
    Ok(Some(manifest))
}

#[cfg(test)]
mod tests {
    use super::load;

    #[test]
    fn test_usage() {
        let _ = load;
    }
}
