pub mod binding;
pub mod buffers;
pub mod compute;
pub mod entries;
pub mod label;
pub mod pipeline;
pub mod plugin;

pub mod prelude {
    pub use super::binding::ShaderDataDetails;
    pub use super::buffers::*;
    pub use super::entries::ShaderEntry;
    pub use super::pipeline::ComputePipeline;
    pub use super::plugin::ShaderPlugin;
    pub use crate::ImageBuilder;
    pub use crate::builders::texture_dimension::*;
}
