pub mod builders;
pub mod internals;

pub use crate::builders::*;
pub use bevy_shader_macros::*;

pub mod prelude {
    // pub use crate::internals::buffers::{BufferReader, BufferWriter};
    pub use crate::{
        ImageBuilder, ImageData, ShaderBuilder,
        internals::{entries::{Dispatch, Entry}, buffers::{ReadableBuffer, WriteableBuffer}},
    };
}
