use std::{hash::Hash, marker::PhantomData, sync::Arc};

use bevy::{
    app::{App, Plugin, PreStartup},
    asset::Assets,
    image::Image,
    prelude::{not, resource_exists, Commands, FromWorld, IntoSystemConfigs, ResMut, Resource},
    render::{
        extract_resource::ExtractResource, render_graph::{RenderGraph, RenderLabel}, storage::ShaderStorageBuffer, Render, RenderSet
    },
};

use crate::{BuildableShader, ShaderBuilder};

use super::{
    binding::{GenericBindGroup, prepare_bind_group},
    buffers::GroupedBuffers,
    compute::ComputeNode,
    entries::{Dispatch, ShaderEntry},
    label::ShaderLabel,
    pipeline::Pipeline,
};

pub struct ShaderPlugin<DataTy, EntriesTy, BuffersTy, PipelineTy, ShaderLabel, const B: usize> {
    initial_data: Arc<DataTy>,
    entry_dispatches: Dispatch<EntriesTy>,
    _buffers_phantom: PhantomData<BuffersTy>,
    _pipeline_phantom: PhantomData<PipelineTy>,
    _label_phantom: PhantomData<ShaderLabel>,
}

impl<
    const B: usize,
    DataTy: Send + Sync + 'static + Clone,
    EntriesTy: Send + Sync + 'static + ShaderEntry + Clone,
    BuffersTy: Send + Sync + 'static + GroupedBuffers<DataTy, B> + Resource + ExtractResource,
    PipelineTy: Send + Sync + 'static + Pipeline + Resource + FromWorld,
    ShaderTy: Send + Sync + 'static + RenderLabel + Clone + Eq + PartialEq + Hash,
> Plugin for ShaderPlugin<DataTy, EntriesTy, BuffersTy, PipelineTy, ShaderTy, B>
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
        render_app.init_resource::<PipelineTy>().add_systems(
            Render,
            prepare_bind_group::<B, _, PipelineTy, BuffersTy>
                .in_set(RenderSet::PrepareBindGroups)
                .run_if(not(resource_exists::<GenericBindGroup<PipelineTy>>)),
        );

        render_app
            .world_mut()
            .resource_mut::<RenderGraph>()
            .add_node(
                ShaderLabel::<ShaderTy>::new(),
                ComputeNode::<PipelineTy, EntriesTy>::new(self.entry_dispatches.clone()),
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

impl<const B: usize, DataTy, EntriesTy, BuffersTy, PipelineTy, ShaderLabel>
    BuildableShader<DataTy, EntriesTy>
    for ShaderPlugin<DataTy, EntriesTy, BuffersTy, PipelineTy, ShaderLabel, B>
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
            _pipeline_phantom: PhantomData,
            _label_phantom: PhantomData,
        }
    }
}
