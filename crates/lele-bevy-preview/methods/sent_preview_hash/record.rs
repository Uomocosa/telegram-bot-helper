use std::path::Path;

use crate::Error;
use crate::deliver;

pub fn record(out_dir: &Path, artifact: &deliver::Artifact) -> Result<(), Error> {
    let cache = deliver::SentPreviewHash::cache_dir(out_dir);
    std::fs::create_dir_all(&cache).map_err(Error::Io)?;
    let dest = deliver::SentPreviewHash::sent_path(out_dir, artifact);
    let tmp = cache.join(format!(
        ".{}.tmp.{}-{:?}",
        artifact.file_name(),
        std::process::id(),
        std::thread::current().id()
    ));
    std::fs::write(&tmp, format!("{}\n", artifact.pixel_hash)).map_err(Error::Io)?;
    std::fs::rename(&tmp, &dest).map_err(Error::Io)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::record;

    #[test]
    fn test_usage() {
        let _ = record;
    }
}
