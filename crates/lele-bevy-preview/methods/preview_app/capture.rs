use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::sync::mpsc;

use bevy::image::Image;
use bevy::prelude::*;
use bevy::render::view::screenshot::{Screenshot, ScreenshotCaptured};

use crate::Error;
use crate::PreviewApp;
use crate::preview;
use crate::preview::basic::constants::PUMP_FRAMES;

pub fn capture(
    app: &mut PreviewApp,
    out: &Path,
    budget: u32,
) -> Result<preview::CapturedImage, Error> {
    let staging = prepare_destination(out)?;
    let Some(image) = request(app, app.target.clone(), &staging, budget) else {
        settle(app);
        return Err(Error::NoFrame(budget));
    };
    let pixels = image.data.clone().unwrap_or_default();
    if pixels.is_empty() {
        return Err(Error::BlankFrame(out.display().to_string()));
    }
    std::fs::rename(&staging, out).map_err(|error| Error::Io(error.to_string()))?;
    Ok(preview::CapturedImage(image))
}

// needed helper: one Screenshot request, pumped until its observer reports a frame
fn request(app: &mut PreviewApp, target: Handle<Image>, out: &Path, budget: u32) -> Option<Image> {
    let (sender, receiver) = mpsc::channel::<Image>();
    let destination = out.to_path_buf();
    app.app
        .world_mut()
        .spawn(Screenshot::image(target))
        .observe(move |event: On<ScreenshotCaptured>| {
            let _ = sender.send(event.image.clone());
        })
        .observe(bevy::render::view::screenshot::save_to_disk(destination));
    let frames = budget.min(PUMP_FRAMES);
    for _ in 0..frames {
        app.app.update();
        if let Ok(image) = receiver.try_recv() {
            return Some(image);
        }
    }
    None
}

// needed helper: a screenshot can be refused while another is in flight
fn settle(app: &mut PreviewApp) {
    for _ in 0..PUMP_FRAMES {
        app.app.update();
    }
}

// needed helper: per-process staging file renamed over `out`, so parallel runs never see it missing
fn prepare_destination(out: &Path) -> Result<PathBuf, Error> {
    let parent = out.parent().map(Path::to_path_buf);
    if let Some(parent) = parent {
        std::fs::create_dir_all(&parent).map_err(|error| Error::Io(error.to_string()))?;
    }
    let extension = out.extension().and_then(OsStr::to_str).unwrap_or("png");
    let staging = out.with_extension(format!("{}.staging.{extension}", std::process::id()));
    let _ = std::fs::remove_file(&staging);
    Ok(staging)
}

#[cfg(test)]
mod tests {
    use super::capture;

    #[test]
    fn test_usage() {
        let _ = capture;
    }
}
