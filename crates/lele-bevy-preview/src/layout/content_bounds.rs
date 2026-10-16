use std::collections::HashMap;
use std::path::Path;

use crate::Error;
use crate::layout;

pub fn content_bounds(path: &Path) -> Result<Option<layout::Bounds>, Error> {
    let image = image::open(path)
        .map_err(|error| Error::Io(format!("{}: {error}", path.display())))?
        .to_rgba8();
    let Some(background) = most_common(&image) else {
        return Ok(None);
    };
    let mut bounds: Option<layout::Bounds> = None;
    for (x, y, pixel) in image.enumerate_pixels() {
        if *pixel == background {
            continue;
        }
        let first = layout::Bounds {
            min_x: x,
            min_y: y,
            max_x: x,
            max_y: y,
            width: image.width(),
            height: image.height(),
        };
        bounds = Some(bounds.map_or(first, |seen| layout::Bounds {
            min_x: seen.min_x.min(x),
            min_y: seen.min_y.min(y),
            max_x: seen.max_x.max(x),
            max_y: seen.max_y.max(y),
            ..seen
        }));
    }
    Ok(bounds)
}

// needed helper: the background is the colour most pixels share, wherever the content sits
fn most_common(image: &image::RgbaImage) -> Option<image::Rgba<u8>> {
    let mut counts: HashMap<image::Rgba<u8>, usize> = HashMap::new();
    for pixel in image.pixels() {
        let count = counts.entry(*pixel).or_insert(0);
        *count = count.saturating_add(1);
    }
    counts
        .into_iter()
        .max_by_key(|(_, count)| *count)
        .map(|(pixel, _)| pixel)
}

#[cfg(test)]
mod tests {
    use super::content_bounds;

    #[test]
    fn test_usage() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("box.png");
        let mut canvas = image::RgbaImage::from_pixel(20, 10, image::Rgba([0, 0, 0, 255]));
        for x in 4..8 {
            for y in 2..5 {
                canvas.put_pixel(x, y, image::Rgba([255, 0, 0, 255]));
            }
        }
        canvas.save(&path).unwrap();
        let bounds = content_bounds(&path).unwrap().unwrap();
        assert_eq!(
            (bounds.min_x, bounds.min_y, bounds.max_x, bounds.max_y),
            (4, 2, 7, 4)
        );
        assert_eq!((bounds.width, bounds.height), (20, 10));

        let blank = dir.path().join("blank.png");
        image::RgbaImage::from_pixel(4, 4, image::Rgba([9, 9, 9, 255]))
            .save(&blank)
            .unwrap();
        assert_eq!(content_bounds(&blank).unwrap(), None);
    }
}
