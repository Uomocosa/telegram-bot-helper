use std::collections::BTreeMap;
use std::path::Path;

use crate::Error;
use crate::deliver;
use crate::deliver::basic::constants;

pub fn write_all(dir: &Path, crate_name: &str, captures: &[deliver::Capture]) -> Result<(), Error> {
    let manifest = deliver::Manifest {
        crate_name: crate_name.to_string(),
        artifacts: captures
            .iter()
            .map(|capture| {
                (
                    capture.artifact.fingerprint.clone(),
                    capture.artifact.clone(),
                )
            })
            .collect::<BTreeMap<_, _>>(),
    };
    let json = serde_json::to_string_pretty(&manifest)?;
    std::fs::create_dir_all(dir).map_err(Error::Io)?;
    std::fs::write(dir.join(constants::MANIFEST_FILE), json).map_err(Error::Io)?;
    std::fs::write(
        dir.join(constants::CHANGES_FILE),
        deliver::Capture::changes_markdown(captures),
    )
    .map_err(Error::Io)
}

#[cfg(test)]
mod tests {
    use super::write_all;

    #[test]
    fn test_usage() {
        let _ = write_all;
    }
}
