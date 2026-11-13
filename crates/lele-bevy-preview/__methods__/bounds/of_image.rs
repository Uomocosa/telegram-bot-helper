use std::path::Path;

use crate::Error;
use crate::layout;

pub fn of_image(path: &Path) -> Result<Option<layout::Bounds>, Error> {
    let image = image::open(path)
        .map_err(|source| Error::Image {
            path: path.to_path_buf(),
            source,
        })?
        .to_rgba8();
    let Some(background) = layout::Bounds::background_colour(&image) else {
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

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::of_image;
    use crate::layout;

    fn write(path: &Path, paint: impl Fn(u32, u32) -> image::Rgba<u8>) {
        let mut image = image::RgbaImage::new(10, 10);
        for (x, y, pixel) in image.enumerate_pixels_mut() {
            *pixel = paint(x, y);
        }
        image.save(path).unwrap();
    }

    #[test]
    fn test_usage() {
        let dir = tempfile::tempdir().unwrap();
        let white = image::Rgba([255, 255, 255, 255]);
        let black = image::Rgba([0, 0, 0, 255]);
        let boxed = dir.path().join("boxed.png");
        write(&boxed, |x, y| {
            if (2..=4).contains(&x) && (3..=6).contains(&y) {
                black
            } else {
                white
            }
        });
        assert_eq!(
            of_image(&boxed).unwrap(),
            Some(layout::Bounds {
                min_x: 2,
                min_y: 3,
                max_x: 4,
                max_y: 6,
                width: 10,
                height: 10,
            })
        );
        let plain = dir.path().join("plain.png");
        write(&plain, |_, _| white);
        assert_eq!(of_image(&plain).unwrap(), None);
    }
}
