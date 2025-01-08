pub mod binding;
pub mod buffers;
pub mod compute;
pub mod entries;
pub mod label;
pub mod pipeline;
pub mod plugin;

pub mod prelude {
    pub use super::buffers::*;
    pub use super::entries::ShaderEntry;
    pub use super::plugin::ShaderPlugin;
    pub use crate::ImageBuilder;
    pub use crate::texture_details::*;

    pub use bevy_image::Image;
    pub use bevy_math::*;
    pub use bevy_render::{
        extract_resource::ExtractResource, render_resource::ShaderType,
        storage::ShaderStorageBuffer,
    };
}
