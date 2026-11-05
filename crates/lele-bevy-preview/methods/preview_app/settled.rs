use std::path::Path;

use crate::Error;
use crate::PreviewApp;
use crate::deliver;
use crate::preview;
use crate::preview::basic::constants::SETTLE_CAPTURES;

pub fn settled(
    app: &mut PreviewApp,
    out: &Path,
    budget: u32,
    min_distinct_colors: usize,
) -> Result<preview::CapturedImage, Error> {
    let mut last: Option<preview::CapturedImage> = None;
    for _ in 0..SETTLE_CAPTURES {
        let Ok(image) = app.capture(out, budget) else {
            continue;
        };
        if image.bytes().is_empty() {
            return Err(Error::BlankFrame(out.display().to_string()));
        }
        image.ensure_visual(out, min_distinct_colors)?;
        let stable = last
            .as_ref()
            .is_some_and(|previous| previous.bytes() == image.bytes());
        if stable {
            return Ok(image);
        }
        last = Some(image);
    }
    let out_dir = out
        .parent()
        .map(std::path::Path::to_path_buf)
        .unwrap_or_default();
    let key = out
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or_default()
        .to_string();
    let scene_name = out
        .file_stem()
        .and_then(|stem| stem.to_str())
        .unwrap_or_default()
        .to_string();
    tracing::warn!(path = %out.display(), "preview settled without a stable frame");
    let entry = deliver::PreviewDecisionLog {
        time: deliver::PreviewDecisionLog::current_time(),
        run: deliver::PreviewDecisionLog::current_run(),
        pid: std::process::id(),
        test_scene: scene_name,
        key,
        status: String::from("unknown"),
        old_hash: String::new(),
        new_hash: String::new(),
        baseline: String::from("missing"),
        driver: deliver::PreviewDecisionLog::current_driver(),
        outcome: String::from("unsettled"),
    };
    let _ = entry.append(&out_dir);
    last.ok_or(Error::NoFrame(SETTLE_CAPTURES))
}

#[cfg(test)]
mod tests {
    use super::settled;

    #[test]
    fn test_usage() {
        let _ = settled;
    }
}
