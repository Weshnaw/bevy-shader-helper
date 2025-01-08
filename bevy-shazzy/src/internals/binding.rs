use std::marker::PhantomData;

use bevy_ecs::system::{Commands, Res, Resource};
use bevy_render::{
    render_asset::RenderAssets, render_resource, renderer::RenderDevice,
    storage::GpuShaderStorageBuffer, texture::GpuImage,
};

use super::{buffers::BufferGroup, pipeline::Pipeline};

pub(super) fn prepare_bind_group<
    const B: usize,
    const E: usize,
    // BuffersDataTy: Clone,
    PipelineTy: Resource + Pipeline,
    BuffersTy: Resource + BufferGroup<B, E>,
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
        BuffersTy::bind_group_label(),
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
