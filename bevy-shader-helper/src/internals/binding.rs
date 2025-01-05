use std::{borrow::Cow, marker::PhantomData};

use bevy::{
    asset::Handle,
    prelude::{Commands, Res, Resource, Shader},
    render::{
        render_asset::RenderAssets,
        render_resource::{
            self, BindGroupLayout, BindGroupLayoutEntries, CachedComputePipelineId,
            ComputePipelineDescriptor, PipelineCache, ShaderStages,
        },
        renderer::RenderDevice,
        storage::GpuShaderStorageBuffer,
        texture::GpuImage,
    },
};

use super::{buffers::GroupedBuffers, pipeline::Pipeline};

pub use bevy_shader_macros::ShaderDataDetails;
pub trait ShaderDataDetails<const B: usize, const E: usize> {
    fn buffer_entries(stage: ShaderStages) -> BindGroupLayoutEntries<B>;

    fn bind_group_label() -> Option<&'static str> {
        None
    }

    fn entries(
        pipeline_cache: &PipelineCache,
        layout: BindGroupLayout,
        shader: Handle<Shader>,
    ) -> [CachedComputePipelineId; E];

    fn create_entry(
        pipeline_cache: &PipelineCache,
        layout: BindGroupLayout,
        shader: Handle<Shader>,
        entry: &'static str,
        label: Option<Cow<'static, str>>,
    ) -> CachedComputePipelineId {
        pipeline_cache.queue_compute_pipeline(ComputePipelineDescriptor {
            label,
            layout: vec![layout],
            push_constant_ranges: vec![],
            shader,
            shader_defs: vec![],
            entry_point: entry.into(),
            zero_initialize_workgroup_memory: false,
        })
    }
}

pub(super) fn prepare_bind_group<
    const B: usize,
    BuffersDataTy: Clone,
    PipelineTy: Resource + Pipeline,
    BuffersTy: Resource + GroupedBuffers<BuffersDataTy, B>,
>(
    mut commands: Commands,
    render_device: Res<RenderDevice>,
    pipeline: Res<PipelineTy>,
    buffer: Res<BuffersTy>,
    buffers: Res<RenderAssets<GpuShaderStorageBuffer>>,
    images: Res<RenderAssets<GpuImage>>,
) {
    // debug!("Preparing bind group");
    let bind_group = render_device.create_bind_group(
        BuffersTy::label(),
        pipeline.layout(),
        &buffer.get_bindings(&buffers, &images),
    );

    let bind_group: GenericBindGroup<PipelineTy> = GenericBindGroup::from_bind_group(bind_group);
    commands.insert_resource(bind_group);
}

#[derive(Resource)]
pub(super) struct GenericBindGroup<T>(pub(super) render_resource::BindGroup, PhantomData<T>);

impl<T> GenericBindGroup<T> {
    fn from_bind_group(bind_group: render_resource::BindGroup) -> Self {
        Self(bind_group, Default::default())
    }
}
