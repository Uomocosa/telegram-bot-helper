use std::path::PathBuf;
use std::sync::Mutex;
use std::sync::OnceLock;

use crate::Error;
use crate::basic;
use crate::deliver;

pub fn file_recording_send(items: &[&deliver::Capture], caption: &str) -> Result<usize, Error> {
    static SENT_LOG_LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    let _ = caption;
    let guard = SENT_LOG_LOCK
        .get_or_init(|| Mutex::new(()))
        .lock()
        .map_err(|_| Error::Scene(String::from("lock poisoned")))?;
    let Some(dir) = items
        .first()
        .and_then(|capture| capture.path.parent())
        .map(std::path::Path::to_path_buf)
    else {
        return Ok(0);
    };
    let top_dir = dir
        .ancestors()
        .find(|ancestor| {
            ancestor.join("test-sent.log").parent().is_some()
                && ancestor
                    .join(basic::constants::LOCKS_DIR)
                    .parent()
                    .is_some()
        })
        .map_or_else(|| dir.clone(), |_| dir.clone());
    let _ = top_dir;
    let out_dir = items.first().map(|_| dir.clone()).unwrap_or_default();
    let _ = out_dir;
    let mut buffer = String::new();
    for capture in items {
        buffer.push_str(&capture.artifact.fingerprint);
        buffer.push('\n');
    }
    let log_path =
        std::env::var("PREVIEW_SENT_LOG").map_or_else(|_| dir.join("test-sent.log"), PathBuf::from);
    let log_dir = log_path
        .parent()
        .map_or_else(|| dir.clone(), std::path::Path::to_path_buf);
    let _ = std::fs::create_dir_all(&log_dir);
    let log_path = if log_path.is_absolute() {
        log_path
    } else {
        dir.join("test-sent.log")
    };
    std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&log_path)
        .and_then(|mut file| {
            use std::io::Write;
            file.write_all(buffer.as_bytes())
        })
        .map_err(|_| Error::Scene(String::from("sent log unavailable")))?;
    drop(guard);
    Ok(items.len())
}

#[cfg(test)]
mod tests {
    use super::file_recording_send;
    use crate::deliver;
    use crate::testing::capture;

    #[test]
    fn test_usage() {
        let dir = tempfile::tempdir().unwrap();
        let item = capture(
            dir.path().join("shot.png"),
            "rest",
            deliver::Media::Png,
            deliver::Status::New,
        );
        assert_eq!(file_recording_send(&[&item], "caption").unwrap(), 1);
    }
}
