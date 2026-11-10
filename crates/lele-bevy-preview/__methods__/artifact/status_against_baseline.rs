use std::path::Path;

use crate::Error;
use crate::deliver;

pub fn status_against_baseline(
    artifact: &deliver::Artifact,
    out_dir: &Path,
) -> Result<deliver::Status, Error> {
    let delivered = artifact.delivered_path(out_dir);
    if !delivered.exists() {
        return Ok(deliver::Status::New);
    }
    let baseline = deliver::Artifact::hash_png_pixels(&delivered).map_err(|error| match error {
        Error::Image { path, source } => Error::Image { path, source },
        Error::Io(io) => Error::DeliveredPreviewUnreadable {
            path: delivered.clone(),
            source: io,
        },
        other => other,
    })?;
    if baseline == artifact.pixel_hash {
        Ok(deliver::Status::Same)
    } else {
        Ok(deliver::Status::Changed)
    }
}

#[cfg(test)]
mod tests {
    use super::status_against_baseline;

    #[test]
    fn test_usage() {
        let _ = status_against_baseline;
    }
}
