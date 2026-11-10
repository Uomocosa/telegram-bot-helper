const DEFAULT_FFMPEG: &str = "ffmpeg";
const BINARY_OVERRIDE_VAR: &str = "LELE_FFMPEG";

#[must_use]
pub fn ffmpeg_binary() -> String {
    let configured = std::env::var(BINARY_OVERRIDE_VAR);
    match configured {
        Ok(path) if !path.trim().is_empty() => path,
        _ => String::from(DEFAULT_FFMPEG),
    }
}

#[cfg(test)]
mod tests {
    use super::ffmpeg_binary;

    #[test]
    fn test_usage() {
        let binary = ffmpeg_binary();
        assert_ne!(binary.trim(), "");
        if std::env::var("LELE_FFMPEG").is_err() {
            assert_eq!(binary, "ffmpeg");
        }
    }
}
