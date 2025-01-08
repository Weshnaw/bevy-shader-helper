use std::{marker::PhantomData, sync::Arc};

use bevy_app::{App, Plugin, PreStartup};
use bevy_ecs::{
    prelude::{not, resource_exists},
    schedule::IntoSystemConfigs,
    system::Resource,
};
use bevy_render::{
    Render, RenderApp, RenderSet, extract_resource::ExtractResource, render_graph::RenderGraph,
};

use crate::{BuildableShader, ShaderBuilder};

use super::{
    binding::{GenericBindGroup, prepare_bind_group},
    buffers::BufferGroup,
    compute::ComputeNode,
    entries::Dispatch,
    label::ShaderLabel,
    pipeline::ComputePipeline,
};

pub struct ShaderPlugin<BuffersTy: BufferGroup<B, E>, const B: usize, const E: usize> {
    initial_data: Arc<BuffersTy::Initializer>,
    entry_dispatches: Dispatch,
    _buffers_phantom: PhantomData<BuffersTy>,
}

impl<
    const B: usize,
    const E: usize,
    BuffersTy: Send + Sync + 'static + BufferGroup<B, E> + Resource + ExtractResource,
> Plugin for ShaderPlugin<BuffersTy, B, E>
{
    fn build(&self, app: &mut App) {
        BuffersTy::create_resource_extractor_plugins(app);
        app.add_systems(
            PreStartup,
            BuffersTy::create_setup(self.initial_data.clone()),
        );
    }

    fn finish(&self, app: &mut App) {
        let render_app = app.sub_app_mut(RenderApp);
        // debug!("Preparing render resources");
        render_app
            .init_resource::<ComputePipeline<B, E, BuffersTy>>()
            .add_systems(
                Render,
                prepare_bind_group::<B, E, ComputePipeline<B, E, BuffersTy>, BuffersTy>
                    .in_set(RenderSet::PrepareBindGroups)
                    .run_if(not(resource_exists::<
                        GenericBindGroup<ComputePipeline<B, E, BuffersTy>>,
                    >)),
            );

        render_app
            .world_mut()
            .resource_mut::<RenderGraph>()
            .add_node(
                ShaderLabel::<BuffersTy>::new(),
                ComputeNode::<ComputePipeline<B, E, BuffersTy>>::new(self.entry_dispatches.clone()),
            );
    }
}

impl<const B: usize, const E: usize, BuffersTy: BufferGroup<B, E>> BuildableShader<BuffersTy, B, E>
    for ShaderPlugin<BuffersTy, B, E>
where
    BuffersTy::Initializer: Default,
{
    fn from_builder(builder: ShaderBuilder<Self, BuffersTy, B, E>) -> Self {
        let initial_data = builder.initial_data.unwrap_or_default();

        let entry_dispatches = builder.dispatches.unwrap_or_default();

        Self {
            initial_data: Arc::new(initial_data),
            entry_dispatches,
            _buffers_phantom: PhantomData,
        }
    }
}
