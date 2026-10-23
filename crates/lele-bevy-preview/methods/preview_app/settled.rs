use std::path::Path;

use crate::Error;
use crate::PreviewApp;
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
