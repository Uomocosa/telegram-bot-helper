use std::path::Path;

use crate::Error;
use crate::PreviewApp;
use crate::preview;

pub fn capture(
    app: &mut PreviewApp,
    out: &Path,
    budget: u32,
) -> Result<preview::CapturedImage, Error> {
    let staging = PreviewApp::staging_path_for(out)?;
    let target = app.target.clone();
    let Some(image) = app.request_screenshot(target, &staging, budget) else {
        app.drain_in_flight_screenshot();
        return Err(Error::NoFrame(budget));
    };
    let pixels = image.data.clone().unwrap_or_default();
    if pixels.is_empty() {
        return Err(Error::BlankFrame(out.display().to_string()));
    }
    std::fs::rename(&staging, out).map_err(Error::Io)?;
    Ok(preview::CapturedImage(image))
}

#[cfg(test)]
mod tests {
    use super::capture;

    #[test]
    fn test_usage() {
        let _ = capture;
    }
}
