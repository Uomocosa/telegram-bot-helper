use std::path::Path;
use std::sync::mpsc;

use bevy::image::Image;
use bevy::prelude::*;
use bevy::render::view::screenshot::{Screenshot, ScreenshotCaptured};

use crate::PreviewApp;
use crate::basic::constants::PUMP_FRAMES;

pub fn request_screenshot(
    app: &mut PreviewApp,
    target: Handle<Image>,
    out: &Path,
    budget: u32,
) -> Option<Image> {
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

#[cfg(test)]
mod tests {
    use super::request_screenshot;

    #[test]
    fn test_usage() {
        let _ = request_screenshot;
    }
}
