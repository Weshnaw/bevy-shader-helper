use bevy_render::render_resource::{TextureDimension, TextureFormat};

// TODO: implement8
pub(crate) trait ToTextureDimension {
    fn texture_dimension() -> TextureDimension;
}

pub struct D1;
pub struct D2;
impl ToTextureDimension for D2 {
    fn texture_dimension() -> TextureDimension {
        TextureDimension::D2
    }
}
pub struct D3;

// TODO implement
pub(crate) trait ToTextureFormat {
    fn texture_format() -> TextureFormat;
}

pub struct R32Float;

impl ToTextureFormat for R32Float {
    fn texture_format() -> TextureFormat {
        TextureFormat::R32Float
    }
}