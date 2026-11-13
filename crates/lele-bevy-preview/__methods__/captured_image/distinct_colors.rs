use std::collections::HashSet;

use crate::preview;

#[must_use]
pub fn distinct_colors(image: &preview::CapturedImage) -> usize {
    let mut seen: HashSet<u32> = HashSet::new();
    let (chunks, _) = image.bytes().as_chunks::<4>();
    for pixel in chunks {
        seen.insert(u32::from_be_bytes(*pixel));
    }
    seen.len()
}

#[cfg(test)]
mod tests {
    use bevy::asset::RenderAssetUsages;
    use bevy::image::Image;
    use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};

    use super::distinct_colors;
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
        let mixed = captured(&[[1, 2, 3, 255], [1, 2, 3, 255], [9, 9, 9, 255], [0, 0, 0, 0]]);
        assert_eq!(distinct_colors(&mixed), 3);
        let solid = captured(&[[5, 5, 5, 255], [5, 5, 5, 255]]);
        assert_eq!(distinct_colors(&solid), 1);
    }
}
