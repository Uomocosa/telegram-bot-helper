use std::path::Path;

use crate::Error;
use crate::deliver;

pub fn status_in_cache(
    artifact: &deliver::Artifact,
    out_dir: &Path,
) -> Result<deliver::Status, Error> {
    let preview = out_dir.join(artifact.file_name());
    if !preview.exists() {
        return Ok(deliver::Status::New);
    }
    let recorded = deliver::SentPreviewHash::load_for(out_dir, artifact)?;
    match recorded {
        None => Ok(deliver::Status::New),
        Some(sent) if sent.as_str() == artifact.pixel_hash.as_str() => Ok(deliver::Status::Same),
        Some(_) => Ok(deliver::Status::Changed),
    }
}

#[cfg(test)]
mod tests {
    use super::status_in_cache;

    #[test]
    fn test_usage() {
        let _ = status_in_cache;
    }
}
