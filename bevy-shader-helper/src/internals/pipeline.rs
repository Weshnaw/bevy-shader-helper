use std::marker::PhantomData;

use bevy::{
    asset::DirectAssetAccessExt,
    prelude::{FromWorld, Resource},
    render::render_resource::{
        BindGroupLayout, CachedComputePipelineId, PipelineCache, ShaderStages,
    },
};

use super::{binding::ShaderDetails, entries::ShaderEntry};

pub trait Pipeline {
    fn label() -> Option<&'static str> {
        None
    }
    fn layout(&self) -> &BindGroupLayout;
    fn get_id<EntryTy: ShaderEntry>(&self, entry: &EntryTy) -> CachedComputePipelineId;
}

#[derive(Resource)]
pub struct ComputePipeline<const B: usize, const E: usize, DataTy> {
    pub layout: BindGroupLayout,
    pub entries: [CachedComputePipelineId; E],
    _phantom: PhantomData<DataTy>,
}

impl<const B: usize, const E: usize, DataTy> Pipeline for ComputePipeline<B, E, DataTy> {
    fn layout(&self) -> &BindGroupLayout {
        &self.layout
    }

    fn get_id<EntryTy: ShaderEntry>(&self, entry: &EntryTy) -> CachedComputePipelineId {
        self.entries[entry.as_key()]
    }
}

impl<const B: usize, const E: usize, DataTy: ShaderDetails<B, E>> FromWorld
    for ComputePipeline<B, E, DataTy>
{
    fn from_world(world: &mut bevy::prelude::World) -> Self {
        let render_device = world.resource::<bevy::render::renderer::RenderDevice>();
        let layout = render_device.create_bind_group_layout(
            DataTy::bind_group_label(),
            &DataTy::buffer_entries(ShaderStages::COMPUTE),
        );

        let shader = world.load_asset("shaders/hello.wgsl");
        let pipeline_cache = world.resource::<PipelineCache>();
        let entries = DataTy::entries(pipeline_cache, layout.clone(), shader);
        Self {
            layout,
            entries,
            _phantom: Default::default(),
        }
    }
}
