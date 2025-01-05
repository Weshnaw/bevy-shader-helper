use std::marker::PhantomData;

use bevy::{
    prelude::Resource,
    render::{
        render_graph,
        render_resource::{ComputePassDescriptor, PipelineCache},
    },
    utils::default,
};

use super::{binding::GenericBindGroup, entries::{Dispatch, ShaderEntry}, pipeline::Pipeline};

#[derive(Default)]
pub(super) enum ShaderStage {
    #[default]
    Loading,
    Startup,
    Update, // TODO: somehow allow for end user to do fancy state things such as the gol example of buffer swapping
}

pub(super) struct ComputeNode<PipelineTy, EntryTy> {
    state: ShaderStage,
    dispatches: Dispatch<EntryTy>,
    _phantom: PhantomData<PipelineTy>,
}

impl<PipelineTy, EntryTy> ComputeNode<PipelineTy, EntryTy> {
    pub(super) fn new(dispatches: Dispatch<EntryTy>) -> Self {
        Self {
            state: ShaderStage::Loading,
            dispatches,
            _phantom: Default::default(),
        }
    }
}

impl<PipelineTy: Resource + Pipeline, EntryTy: ShaderEntry + Send + Sync + 'static>
    render_graph::Node for ComputeNode<PipelineTy, EntryTy>
{
    fn update(&mut self, world: &mut bevy::prelude::World) {
        let pipeline_cache = world.resource::<PipelineCache>();
        let pipeline = world.resource::<PipelineTy>();

        match self.state {
            ShaderStage::Loading => {
                if self.dispatches.on_startup_success(pipeline_cache, pipeline) {
                    self.state = ShaderStage::Startup
                }
            }
            ShaderStage::Startup => {
                if self.dispatches.on_update_success(pipeline_cache, pipeline) {
                    self.state = ShaderStage::Update
                }
            }
            _ => {}
        }
    }

    fn run(
        &self,
        _graph: &mut bevy::render::render_graph::RenderGraphContext,
        render_context: &mut bevy::render::renderer::RenderContext,
        world: &bevy::prelude::World,
    ) -> Result<(), bevy::render::render_graph::NodeRunError> {
        let pipeline_cache = world.resource::<PipelineCache>();
        let pipeline = world.resource::<PipelineTy>();
        let bind_group = world.resource::<GenericBindGroup<PipelineTy>>();
        let mut pass =
            render_context
                .command_encoder()
                .begin_compute_pass(&ComputePassDescriptor {
                    label: PipelineTy::label(),
                    ..default()
                });
        match self.state {
            ShaderStage::Startup => {
                self.dispatches.on_startup_dispatch(
                    pipeline_cache,
                    pipeline,
                    &mut pass,
                    bind_group,
                );
            }
            ShaderStage::Update => {
                self.dispatches
                    .on_update_dispatch(pipeline_cache, pipeline, &mut pass, bind_group);
            }
            _ => {}
        }

        Ok(())
    }
}
