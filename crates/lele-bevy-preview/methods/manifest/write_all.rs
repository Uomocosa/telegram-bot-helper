use std::collections::BTreeMap;
use std::path::Path;

use crate::Error;
use crate::deliver;
use crate::deliver::basic::constants;

pub fn write_all(dir: &Path, crate_name: &str, captures: &[deliver::Capture]) -> Result<(), Error> {
    let _held = deliver::Manifest::acquire_exclusive_lock(dir)?;
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
    deliver::Manifest::write_merged_atomically(dir, &manifest)?;
    let changes = deliver::Capture::changes_markdown(captures);
    let changes_tmp = dir.join(format!("{}.tmp", constants::CHANGES_FILE));
    std::fs::write(&changes_tmp, &changes).map_err(Error::Io)?;
    std::fs::rename(&changes_tmp, dir.join(constants::CHANGES_FILE)).map_err(Error::Io)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::write_all;

    #[test]
    fn test_usage() {
        let _ = write_all;
    }
}
