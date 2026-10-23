use std::collections::HashMap;
use std::path::Path;

use crate::Error;
use crate::layout;

pub fn of_image(path: &Path) -> Result<Option<layout::Bounds>, Error> {
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
    use super::of_image;

    #[test]
    fn test_usage() {
        let _ = of_image;
    }
}
