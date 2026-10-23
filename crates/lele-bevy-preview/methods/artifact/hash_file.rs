use std::path::Path;

use crate::Error;
use crate::deliver;

pub fn hash_file(path: &Path) -> Result<String, Error> {
    let bytes = std::fs::read(path).map_err(Error::Io)?;
    Ok(deliver::Artifact::hash_bytes(&bytes))
}

#[cfg(test)]
mod tests {
    use super::hash_file;

    #[test]
    fn test_usage() {
        let _ = hash_file;
    }
}
