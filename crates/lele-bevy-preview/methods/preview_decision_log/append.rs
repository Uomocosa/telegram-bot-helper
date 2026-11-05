use std::io::Write;

use crate::Error;
use crate::deliver;

pub fn append(log: &deliver::PreviewDecisionLog, out_dir: &std::path::Path) -> Result<(), Error> {
    let path = deliver::PreviewDecisionLog::log_path(out_dir);
    let json = serde_json::to_string(log)?;
    std::fs::create_dir_all(out_dir).map_err(Error::Io)?;
    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .map_err(Error::Io)?;
    file.write_all(format!("{json}\n").as_bytes())
        .map_err(Error::Io)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::append;

    #[test]
    fn test_usage() {
        let _ = append;
    }
}
