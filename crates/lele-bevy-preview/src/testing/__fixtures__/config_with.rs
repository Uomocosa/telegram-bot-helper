use std::path::Path;

use crate::Error;
use crate::deliver;
use crate::preview;

pub fn config_with(
    dir: &Path,
    send: fn(&[&deliver::Capture], &str) -> Result<usize, Error>,
) -> preview::Config {
    preview::Config {
        out_dir: dir.to_path_buf(),
        send_preview_group: send,
        ..preview::Config::default()
    }
}

#[cfg(test)]
mod tests {
    use super::config_with;
    use crate::deliver;
    use std::path::Path;

    #[test]
    fn test_usage() {
        let config = config_with(Path::new("out"), deliver::Capture::send_group);
        assert_eq!(config.out_dir, Path::new("out"));
    }
}
