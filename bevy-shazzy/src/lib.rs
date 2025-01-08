pub mod builders;
pub mod internals;
pub mod texture_details;

pub use crate::builders::*;

pub mod prelude {
    pub use crate::{
        BuildableShader, ImageBuilder, ImageData,
        internals::buffers::{ReadableBuffer, WriteableBuffer},
    };
}

// pub use bevy;

// Re-export some bevy types for the derive macros
pub mod bevy {
    pub use bevy_asset::{Assets, Handle};
    pub use bevy_ecs as ecs;
    pub use bevy_ecs::prelude::{Commands, Resource};
    pub use bevy_image::Image;
    pub use bevy_render as render;
}
