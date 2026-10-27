use bevy::asset::RenderAssetUsages;
use bevy::image::Image;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat, TextureUsages};

use crate::preview::basic::constants::{BACKGROUND_A, BACKGROUND_B, BACKGROUND_G, BACKGROUND_R};

#[must_use]
pub fn offscreen_image(width: u32, height: u32) -> Image {
    let mut image = Image::new_fill(
        Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &[BACKGROUND_R, BACKGROUND_G, BACKGROUND_B, BACKGROUND_A],
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
    );
    image.texture_descriptor.usage |=
        TextureUsages::RENDER_ATTACHMENT | TextureUsages::COPY_SRC | TextureUsages::TEXTURE_BINDING;
    image
}

#[cfg(test)]
mod tests {
    use bevy::render::render_resource::TextureUsages;

    use super::offscreen_image;

    #[test]
    fn test_usage() {
        let image = offscreen_image(64, 32);
        assert_eq!(image.width(), 64);
        assert_eq!(image.height(), 32);
        assert!(
            image
                .texture_descriptor
                .usage
                .contains(TextureUsages::RENDER_ATTACHMENT | TextureUsages::COPY_SRC)
        );
    }
}
