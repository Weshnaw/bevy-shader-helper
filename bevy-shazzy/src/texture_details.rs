use bevy_render::render_resource::{TextureDimension, TextureFormat};

pub(crate) trait ToTextureDimension {
    fn texture_dimension() -> TextureDimension;
}

pub struct D1;
impl ToTextureDimension for D1 {
    fn texture_dimension() -> TextureDimension {
        TextureDimension::D1
    }
}
pub struct D2;
impl ToTextureDimension for D2 {
    fn texture_dimension() -> TextureDimension {
        TextureDimension::D2
    }
}
pub struct D3;
impl ToTextureDimension for D3 {
    fn texture_dimension() -> TextureDimension {
        TextureDimension::D3
    }
}

pub(crate) trait ToTextureFormat {
    fn texture_format() -> TextureFormat;
}

pub struct R8Unorm;
impl ToTextureFormat for R8Unorm {
    fn texture_format() -> TextureFormat {
        TextureFormat::R8Unorm
    }
}

pub struct R8Snorm;
impl ToTextureFormat for R8Snorm {
    fn texture_format() -> TextureFormat {
        TextureFormat::R8Snorm
    }
}

pub struct R8Uint;
impl ToTextureFormat for R8Uint {
    fn texture_format() -> TextureFormat {
        TextureFormat::R8Uint
    }
}

pub struct R8Sint;
impl ToTextureFormat for R8Sint {
    fn texture_format() -> TextureFormat {
        TextureFormat::R8Sint
    }
}

pub struct R16Uint;
impl ToTextureFormat for R16Uint {
    fn texture_format() -> TextureFormat {
        TextureFormat::R16Uint
    }
}

pub struct R16Sint;
impl ToTextureFormat for R16Sint {
    fn texture_format() -> TextureFormat {
        TextureFormat::R16Sint
    }
}

pub struct R16Unorm;
impl ToTextureFormat for R16Unorm {
    fn texture_format() -> TextureFormat {
        TextureFormat::R16Unorm
    }
}

pub struct R16Snorm;
impl ToTextureFormat for R16Snorm {
    fn texture_format() -> TextureFormat {
        TextureFormat::R16Snorm
    }
}

pub struct R16Float;
impl ToTextureFormat for R16Float {
    fn texture_format() -> TextureFormat {
        TextureFormat::R16Float
    }
}

pub struct Rg8Unorm;
impl ToTextureFormat for Rg8Unorm {
    fn texture_format() -> TextureFormat {
        TextureFormat::Rg8Unorm
    }
}

pub struct Rg8Snorm;
impl ToTextureFormat for Rg8Snorm {
    fn texture_format() -> TextureFormat {
        TextureFormat::Rg8Snorm
    }
}

pub struct Rg8Uint;
impl ToTextureFormat for Rg8Uint {
    fn texture_format() -> TextureFormat {
        TextureFormat::Rg8Uint
    }
}

pub struct Rg8Sint;
impl ToTextureFormat for Rg8Sint {
    fn texture_format() -> TextureFormat {
        TextureFormat::Rg8Sint
    }
}

pub struct R32Uint;
impl ToTextureFormat for R32Uint {
    fn texture_format() -> TextureFormat {
        TextureFormat::R32Uint
    }
}

pub struct R32Sint;
impl ToTextureFormat for R32Sint {
    fn texture_format() -> TextureFormat {
        TextureFormat::R32Sint
    }
}

pub struct R32Float;
impl ToTextureFormat for R32Float {
    fn texture_format() -> TextureFormat {
        TextureFormat::R32Float
    }
}

pub struct Rg16Uint;
impl ToTextureFormat for Rg16Uint {
    fn texture_format() -> TextureFormat {
        TextureFormat::Rg16Uint
    }
}

pub struct Rg16Sint;
impl ToTextureFormat for Rg16Sint {
    fn texture_format() -> TextureFormat {
        TextureFormat::Rg16Sint
    }
}

pub struct Rg16Unorm;
impl ToTextureFormat for Rg16Unorm {
    fn texture_format() -> TextureFormat {
        TextureFormat::Rg16Unorm
    }
}

pub struct Rg16Snorm;
impl ToTextureFormat for Rg16Snorm {
    fn texture_format() -> TextureFormat {
        TextureFormat::Rg16Snorm
    }
}

pub struct Rg16Float;
impl ToTextureFormat for Rg16Float {
    fn texture_format() -> TextureFormat {
        TextureFormat::Rg16Float
    }
}

pub struct Rgba8Unorm;
impl ToTextureFormat for Rgba8Unorm {
    fn texture_format() -> TextureFormat {
        TextureFormat::Rgba8Unorm
    }
}

pub struct Rgba8UnormSrgb;
impl ToTextureFormat for Rgba8UnormSrgb {
    fn texture_format() -> TextureFormat {
        TextureFormat::Rgba8UnormSrgb
    }
}

pub struct Rgba8Snorm;
impl ToTextureFormat for Rgba8Snorm {
    fn texture_format() -> TextureFormat {
        TextureFormat::Rgba8Snorm
    }
}

pub struct Rgba8Uint;
impl ToTextureFormat for Rgba8Uint {
    fn texture_format() -> TextureFormat {
        TextureFormat::Rgba8Uint
    }
}

pub struct Rgba8Sint;
impl ToTextureFormat for Rgba8Sint {
    fn texture_format() -> TextureFormat {
        TextureFormat::Rgba8Sint
    }
}

pub struct Bgra8Unorm;
impl ToTextureFormat for Bgra8Unorm {
    fn texture_format() -> TextureFormat {
        TextureFormat::Bgra8Unorm
    }
}

pub struct Bgra8UnormSrgb;
impl ToTextureFormat for Bgra8UnormSrgb {
    fn texture_format() -> TextureFormat {
        TextureFormat::Bgra8UnormSrgb
    }
}

pub struct Rgb9e5Ufloat;
impl ToTextureFormat for Rgb9e5Ufloat {
    fn texture_format() -> TextureFormat {
        TextureFormat::Rgb9e5Ufloat
    }
}

pub struct Rgb10a2Uint;
impl ToTextureFormat for Rgb10a2Uint {
    fn texture_format() -> TextureFormat {
        TextureFormat::Rgb10a2Uint
    }
}

pub struct Rgb10a2Unorm;
impl ToTextureFormat for Rgb10a2Unorm {
    fn texture_format() -> TextureFormat {
        TextureFormat::Rgb10a2Unorm
    }
}

pub struct Rg11b10Ufloat;
impl ToTextureFormat for Rg11b10Ufloat {
    fn texture_format() -> TextureFormat {
        TextureFormat::Rg11b10Ufloat
    }
}

pub struct Rg32Uint;
impl ToTextureFormat for Rg32Uint {
    fn texture_format() -> TextureFormat {
        TextureFormat::Rg32Uint
    }
}

pub struct Rg32Sint;
impl ToTextureFormat for Rg32Sint {
    fn texture_format() -> TextureFormat {
        TextureFormat::Rg32Sint
    }
}

pub struct Rg32Float;
impl ToTextureFormat for Rg32Float {
    fn texture_format() -> TextureFormat {
        TextureFormat::Rg32Float
    }
}

pub struct Rgba16Uint;
impl ToTextureFormat for Rgba16Uint {
    fn texture_format() -> TextureFormat {
        TextureFormat::Rgba16Uint
    }
}

pub struct Rgba16Sint;
impl ToTextureFormat for Rgba16Sint {
    fn texture_format() -> TextureFormat {
        TextureFormat::Rgba16Sint
    }
}

pub struct Rgba16Unorm;
impl ToTextureFormat for Rgba16Unorm {
    fn texture_format() -> TextureFormat {
        TextureFormat::Rgba16Unorm
    }
}

pub struct Rgba16Snorm;
impl ToTextureFormat for Rgba16Snorm {
    fn texture_format() -> TextureFormat {
        TextureFormat::Rgba16Snorm
    }
}

pub struct Rgba16Float;
impl ToTextureFormat for Rgba16Float {
    fn texture_format() -> TextureFormat {
        TextureFormat::Rgba16Float
    }
}

pub struct Rgba32Uint;
impl ToTextureFormat for Rgba32Uint {
    fn texture_format() -> TextureFormat {
        TextureFormat::Rgba32Uint
    }
}

pub struct Rgba32Sint;
impl ToTextureFormat for Rgba32Sint {
    fn texture_format() -> TextureFormat {
        TextureFormat::Rgba32Sint
    }
}

pub struct Rgba32Float;
impl ToTextureFormat for Rgba32Float {
    fn texture_format() -> TextureFormat {
        TextureFormat::Rgba32Float
    }
}

pub struct Stencil8;
impl ToTextureFormat for Stencil8 {
    fn texture_format() -> TextureFormat {
        TextureFormat::Stencil8
    }
}

pub struct Depth16Unorm;
impl ToTextureFormat for Depth16Unorm {
    fn texture_format() -> TextureFormat {
        TextureFormat::Depth16Unorm
    }
}

pub struct Depth24Plus;
impl ToTextureFormat for Depth24Plus {
    fn texture_format() -> TextureFormat {
        TextureFormat::Depth24Plus
    }
}

pub struct Depth24PlusStencil8;
impl ToTextureFormat for Depth24PlusStencil8 {
    fn texture_format() -> TextureFormat {
        TextureFormat::Depth24PlusStencil8
    }
}

pub struct Depth32Float;
impl ToTextureFormat for Depth32Float {
    fn texture_format() -> TextureFormat {
        TextureFormat::Depth32Float
    }
}

pub struct Depth32FloatStencil8;
impl ToTextureFormat for Depth32FloatStencil8 {
    fn texture_format() -> TextureFormat {
        TextureFormat::Depth32FloatStencil8
    }
}

pub struct NV12;
impl ToTextureFormat for NV12 {
    fn texture_format() -> TextureFormat {
        TextureFormat::NV12
    }
}

pub struct Bc1RgbaUnorm;
impl ToTextureFormat for Bc1RgbaUnorm {
    fn texture_format() -> TextureFormat {
        TextureFormat::Bc1RgbaUnorm
    }
}

pub struct Bc1RgbaUnormSrgb;
impl ToTextureFormat for Bc1RgbaUnormSrgb {
    fn texture_format() -> TextureFormat {
        TextureFormat::Bc1RgbaUnormSrgb
    }
}

pub struct Bc2RgbaUnorm;
impl ToTextureFormat for Bc2RgbaUnorm {
    fn texture_format() -> TextureFormat {
        TextureFormat::Bc2RgbaUnorm
    }
}

pub struct Bc2RgbaUnormSrgb;
impl ToTextureFormat for Bc2RgbaUnormSrgb {
    fn texture_format() -> TextureFormat {
        TextureFormat::Bc2RgbaUnormSrgb
    }
}

pub struct Bc3RgbaUnorm;
impl ToTextureFormat for Bc3RgbaUnorm {
    fn texture_format() -> TextureFormat {
        TextureFormat::Bc3RgbaUnorm
    }
}

pub struct Bc3RgbaUnormSrgb;
impl ToTextureFormat for Bc3RgbaUnormSrgb {
    fn texture_format() -> TextureFormat {
        TextureFormat::Bc3RgbaUnormSrgb
    }
}

pub struct Bc4RUnorm;
impl ToTextureFormat for Bc4RUnorm {
    fn texture_format() -> TextureFormat {
        TextureFormat::Bc4RUnorm
    }
}

pub struct Bc4RSnorm;
impl ToTextureFormat for Bc4RSnorm {
    fn texture_format() -> TextureFormat {
        TextureFormat::Bc4RSnorm
    }
}

pub struct Bc5RgUnorm;
impl ToTextureFormat for Bc5RgUnorm {
    fn texture_format() -> TextureFormat {
        TextureFormat::Bc5RgUnorm
    }
}

pub struct Bc5RgSnorm;
impl ToTextureFormat for Bc5RgSnorm {
    fn texture_format() -> TextureFormat {
        TextureFormat::Bc5RgSnorm
    }
}

pub struct Bc6hRgbUfloat;
impl ToTextureFormat for Bc6hRgbUfloat {
    fn texture_format() -> TextureFormat {
        TextureFormat::Bc6hRgbUfloat
    }
}

pub struct Bc6hRgbFloat;
impl ToTextureFormat for Bc6hRgbFloat {
    fn texture_format() -> TextureFormat {
        TextureFormat::Bc6hRgbFloat
    }
}

pub struct Bc7RgbaUnorm;
impl ToTextureFormat for Bc7RgbaUnorm {
    fn texture_format() -> TextureFormat {
        TextureFormat::Bc7RgbaUnorm
    }
}

pub struct Bc7RgbaUnormSrgb;
impl ToTextureFormat for Bc7RgbaUnormSrgb {
    fn texture_format() -> TextureFormat {
        TextureFormat::Bc7RgbaUnormSrgb
    }
}

pub struct Etc2Rgb8Unorm;
impl ToTextureFormat for Etc2Rgb8Unorm {
    fn texture_format() -> TextureFormat {
        TextureFormat::Etc2Rgb8Unorm
    }
}

pub struct Etc2Rgb8UnormSrgb;
impl ToTextureFormat for Etc2Rgb8UnormSrgb {
    fn texture_format() -> TextureFormat {
        TextureFormat::Etc2Rgb8UnormSrgb
    }
}

pub struct Etc2Rgb8A1Unorm;
impl ToTextureFormat for Etc2Rgb8A1Unorm {
    fn texture_format() -> TextureFormat {
        TextureFormat::Etc2Rgb8A1Unorm
    }
}

pub struct Etc2Rgb8A1UnormSrgb;
impl ToTextureFormat for Etc2Rgb8A1UnormSrgb {
    fn texture_format() -> TextureFormat {
        TextureFormat::Etc2Rgb8A1UnormSrgb
    }
}

pub struct Etc2Rgba8Unorm;
impl ToTextureFormat for Etc2Rgba8Unorm {
    fn texture_format() -> TextureFormat {
        TextureFormat::Etc2Rgba8Unorm
    }
}

pub struct Etc2Rgba8UnormSrgb;
impl ToTextureFormat for Etc2Rgba8UnormSrgb {
    fn texture_format() -> TextureFormat {
        TextureFormat::Etc2Rgba8UnormSrgb
    }
}

pub struct EacR11Unorm;
impl ToTextureFormat for EacR11Unorm {
    fn texture_format() -> TextureFormat {
        TextureFormat::EacR11Unorm
    }
}

pub struct EacR11Snorm;
impl ToTextureFormat for EacR11Snorm {
    fn texture_format() -> TextureFormat {
        TextureFormat::EacR11Snorm
    }
}

pub struct EacRg11Unorm;
impl ToTextureFormat for EacRg11Unorm {
    fn texture_format() -> TextureFormat {
        TextureFormat::EacRg11Unorm
    }
}

pub struct EacRg11Snorm;
impl ToTextureFormat for EacRg11Snorm {
    fn texture_format() -> TextureFormat {
        TextureFormat::EacRg11Snorm
    }
}
