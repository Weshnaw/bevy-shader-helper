use std::marker::PhantomData;

use bevy_ecs::{system::Resource, world::World};
use bevy_render::{
    render_graph::{self, NodeRunError, RenderGraphContext},
    render_resource::{ComputePassDescriptor, PipelineCache},
    renderer::RenderContext,
};

use super::{binding::GenericBindGroup, entries::Dispatch, pipeline::Pipeline};

#[derive(Default)]
pub(super) enum ShaderStage {
    #[default]
    Loading,
    Startup,
    Update, // TODO: somehow allow for end user to do fancy state things such as the gol example of buffer swapping
}

pub(super) struct ComputeNode<PipelineTy> {
    state: ShaderStage,
    dispatches: Dispatch,
    _phantom: PhantomData<PipelineTy>,
}

impl<PipelineTy> ComputeNode<PipelineTy> {
    pub(super) fn new(dispatches: Dispatch) -> Self {
        Self {
            state: ShaderStage::Loading,
            dispatches,
            _phantom: Default::default(),
        }
    }
}

impl<PipelineTy: Resource + Pipeline> render_graph::Node for ComputeNode<PipelineTy> {
    fn update(&mut self, world: &mut World) {
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
        _graph: &mut RenderGraphContext,
        render_context: &mut RenderContext,
        world: &World,
    ) -> Result<(), NodeRunError> {
        let pipeline_cache = world.resource::<PipelineCache>();
        let pipeline = world.resource::<PipelineTy>();
        let bind_group = world.resource::<GenericBindGroup<PipelineTy>>();
        let mut pass =
            render_context
                .command_encoder()
                .begin_compute_pass(&ComputePassDescriptor {
                    label: PipelineTy::label(),
                    ..Default::default()
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
