use std::path::Path;

use crate::Error;
use crate::deliver;
use crate::deliver::basic::constants::HASH_CHARS;

pub fn load_for(
    out_dir: &Path,
    artifact: &deliver::Artifact,
) -> Result<Option<deliver::SentPreviewHash>, Error> {
    let path = deliver::SentPreviewHash::sent_path(out_dir, artifact);
    let text = match std::fs::read_to_string(&path) {
        Ok(text) => text,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Err(error) => return Err(Error::Io(error)),
    };
    let trimmed = text.strip_suffix('\n').unwrap_or(&text);
    let trimmed = trimmed.strip_suffix('\r').unwrap_or(trimmed);
    if trimmed.len() != HASH_CHARS
        || !trimmed.bytes().all(|byte| {
            byte.is_ascii_hexdigit() && (byte.is_ascii_digit() || byte.is_ascii_lowercase())
        })
    {
        return Err(Error::SentCacheInvalid { path });
    }
    Ok(Some(deliver::SentPreviewHash(trimmed.to_string())))
}

#[cfg(test)]
mod tests {
    use super::load_for;

    #[test]
    fn test_usage() {
        let _ = load_for;
    }
}
