use bevy::asset::RenderAssetUsages;
use bevy::image::Image;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};

use crate::preview;

pub fn captured(pixels: &[[u8; 4]]) -> preview::CapturedImage {
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

#[cfg(test)]
mod tests {
    use super::captured;

    #[test]
    fn test_usage() {
        assert_eq!(captured(&[[1, 2, 3, 255]]).bytes().len(), 4);
    }
}
