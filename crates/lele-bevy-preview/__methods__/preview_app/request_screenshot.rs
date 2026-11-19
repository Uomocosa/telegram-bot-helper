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
    use crate::PreviewApp;
    use crate::preview;
    use crate::testing::fast_config;
    use crate::testing::scene_with;

    use super::request_screenshot;

    #[test]
    fn test_usage() {
        let dir = tempfile::tempdir().unwrap();
        let mut app = PreviewApp::from_scene(
            &scene_with("demo", &["one room"], None),
            &fast_config(dir.path()),
        );
        let out = dir.path().join("frame.png");
        let target = app.target.clone();
        let image = request_screenshot(&mut app, target, &out, 240);
        let image = image.unwrap();
        assert_eq!(image.width(), preview::DEFAULT_WIDTH);
        assert_eq!(image.height(), preview::DEFAULT_HEIGHT);
    }
}
