use std::marker::PhantomData;

use bevy_asset::DirectAssetAccessExt;
use bevy_ecs::{
    system::Resource,
    world::{FromWorld, World},
};
use bevy_render::{
    render_resource::{BindGroupLayout, CachedComputePipelineId, PipelineCache, ShaderStages},
    renderer::RenderDevice,
};

use super::{buffers::BufferGroup, entries::ShaderEntry};

pub trait Pipeline {
    fn label() -> Option<&'static str> {
        None
    }
    fn layout(&self) -> &BindGroupLayout;
    fn get_id<EntryTy: ShaderEntry>(&self, entry: &EntryTy) -> CachedComputePipelineId;
}

#[derive(Resource)]
pub struct ComputePipeline<const B: usize, const E: usize, BuffersTy> {
    pub layout: BindGroupLayout,
    pub entries: [CachedComputePipelineId; E],
    _phantom: PhantomData<BuffersTy>,
}

impl<const B: usize, const E: usize, BuffersTy> Pipeline for ComputePipeline<B, E, BuffersTy> {
    fn layout(&self) -> &BindGroupLayout {
        &self.layout
    }

    fn get_id<EntryTy: ShaderEntry>(&self, entry: &EntryTy) -> CachedComputePipelineId {
        self.entries[entry.as_key()]
    }
}

impl<const B: usize, const E: usize, BuffersTy: BufferGroup<B, E>> FromWorld
    for ComputePipeline<B, E, BuffersTy>
{
    fn from_world(world: &mut World) -> Self {
        let render_device = world.resource::<RenderDevice>();
        let layout = render_device.create_bind_group_layout(
            BuffersTy::bind_group_label(),
            &BuffersTy::buffer_entries(ShaderStages::COMPUTE),
        );

        let shader = world.load_asset("shaders/hello.wgsl");
        let pipeline_cache = world.resource::<PipelineCache>();
        let entries = BuffersTy::entries(pipeline_cache, layout.clone(), shader);
        Self {
            layout,
            entries,
            _phantom: Default::default(),
        }
    }
}
