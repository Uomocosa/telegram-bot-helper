use std::path::Path;

use crate::Error;
use crate::preview;

pub fn ensure_visual(image: &preview::CapturedImage, out: &Path, min: usize) -> Result<(), Error> {
    let distinct = image.distinct_colors();
    if distinct < min {
        return Err(Error::NoVisual {
            path: out.display().to_string(),
            distinct,
            min,
        });
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use bevy::asset::RenderAssetUsages;
    use bevy::image::Image;
    use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};

    use super::ensure_visual;
    use crate::Error;
    use crate::preview;

    fn captured(pixels: &[[u8; 4]]) -> preview::CapturedImage {
        let data: Vec<u8> = pixels.iter().flatten().copied().collect();
        let width = u32::try_from(pixels.len()).unwrap();
        preview::CapturedImage(Image::new(
            Extent3d {
                width,
                height: 1,
                depth_or_array_layers: 1,
            },
            TextureDimension::D2,
            data,
            TextureFormat::Rgba8UnormSrgb,
            RenderAssetUsages::default(),
        ))
    }

    #[test]
    fn test_usage() {
        let out = Path::new("out.png");
        let image = captured(&[[1, 2, 3, 255], [9, 9, 9, 255], [0, 0, 0, 255]]);
        assert!(ensure_visual(&image, out, 2).is_ok());
        assert!(matches!(
            ensure_visual(&image, out, 4),
            Err(Error::NoVisual {
                distinct: 3,
                min: 4,
                ..
            })
        ));
    }
}
