use std::{fmt, hash::Hash, marker::PhantomData, sync::Arc};

use bevy::{
    app::{App, Plugin, PreStartup},
    asset::Assets,
    image::Image,
    prelude::{not, resource_exists, Commands, IntoSystemConfigs, ResMut, Resource},
    render::{
        extract_resource::ExtractResource, render_graph::RenderGraph, storage::ShaderStorageBuffer, Render, RenderSet
    },
};

use crate::{BuildableShader, ShaderBuilder};

use super::{
    binding::{prepare_bind_group, GenericBindGroup, ShaderDataDetails},
    buffers::GroupedBuffers,
    compute::ComputeNode,
    entries::{Dispatch, ShaderEntry},
    label::ShaderLabel,
    pipeline::ComputePipeline,
};

pub struct ShaderPlugin<DataTy, EntriesTy, BuffersTy, const B: usize, const E: usize> {
    initial_data: Arc<DataTy>,
    entry_dispatches: Dispatch<EntriesTy>,
    _buffers_phantom: PhantomData<BuffersTy>,
}

impl<
    const B: usize,
    const E: usize,
    DataTy: Send + Sync + 'static + Clone + ShaderDataDetails<B, E>,
    EntriesTy: Send + Sync + 'static + ShaderEntry + Clone + Eq + Hash + fmt::Debug,
    BuffersTy: Send + Sync + 'static + GroupedBuffers<DataTy, B> + Resource + ExtractResource,
    // ShaderTy: Send + Sync + 'static + RenderLabel + Clone + Eq + PartialEq + Hash,
> Plugin for ShaderPlugin<DataTy, EntriesTy, BuffersTy, B, E>
{
    fn build(&self, app: &mut App) {
        BuffersTy::create_resource_extractor_plugins(app);
        app.add_systems(
            PreStartup,
            create_setup::<B, DataTy, BuffersTy>(self.initial_data.clone()),
        );
    }

    fn finish(&self, app: &mut App) {
        let render_app = app.sub_app_mut(bevy::render::RenderApp);
        // debug!("Preparing render resources");
        render_app.init_resource::<ComputePipeline<B, E, DataTy>>().add_systems(
            Render,
            prepare_bind_group::<B, _, ComputePipeline<B, E, DataTy>, BuffersTy>
                .in_set(RenderSet::PrepareBindGroups)
                .run_if(not(resource_exists::<GenericBindGroup<ComputePipeline<B, E, DataTy>>>)),
        );

        render_app
            .world_mut()
            .resource_mut::<RenderGraph>()
            .add_node(
                ShaderLabel::<EntriesTy>::new(),
                ComputeNode::<ComputePipeline<B, E, DataTy>, EntriesTy>::new(self.entry_dispatches.clone()),
            );
    }
}

fn create_setup<const B: usize, DataTy: Clone, BuffersTy: GroupedBuffers<DataTy, B>>(
    d: Arc<DataTy>,
) -> impl Fn(Commands, ResMut<Assets<ShaderStorageBuffer>>, ResMut<Assets<Image>>) {
    move |mut commands, mut buffers, mut images| {
        BuffersTy::insert_resources(&mut commands, &mut buffers, &mut images, d.as_ref().clone());
    }
}

impl<const B: usize, const E: usize, DataTy, EntriesTy, BuffersTy>
    BuildableShader<DataTy, EntriesTy>
    for ShaderPlugin<DataTy, EntriesTy, BuffersTy, B, E>
{
    fn from_builder(builder: ShaderBuilder<Self, DataTy, EntriesTy>) -> Self {
        let Some(initial_data) = builder.initial_data else {
            todo!()
        };
        let Some(entry_dispatches) = builder.dispatches else {
            todo!()
        };

        Self {
            initial_data: Arc::new(initial_data),
            entry_dispatches,
            _buffers_phantom: PhantomData,
        }
    }
}
