pub enum ShaderStage {
    Loading,
    Startup,
    Update,
}
#[derive(Clone)]
pub enum ImageData {
    Fill([u8; 4]),
    Data(Vec<u8>),
    Zeros,
}
#[derive(Clone)]
pub struct ImageInit {
    pub size: bevy::render::render_resource::Extent3d,
    pub data: ImageData,
}
#[derive(Clone)]
pub struct Plugin {
    pub initializations: Init,
    pub dispatches: ShaderDispatches,
}
impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut bevy::app::App) {
        if self.dispatches.on_startup.is_empty() && self.dispatches.on_update.is_empty()
        {
            return;
        }
        app.add_plugins(());
        app.add_systems(bevy::app::Startup, create_setup(self.initializations.clone()));
    }
    fn finish(&self, app: &mut bevy::app::App) {
        let dispatches = self.dispatches.clone();
        if dispatches.on_startup.is_empty() && dispatches.on_update.is_empty() {
            return;
        }
        let render_app = app.sub_app_mut(bevy::render::RenderApp);
        render_app.init_resource::<ComputePipeline>();
        render_app
            .add_systems(
                bevy::render::Render,
                bevy::prelude::IntoSystemConfigs::run_if(
                    bevy::prelude::IntoSystemConfigs::in_set(
                        prepare_bind_group,
                        bevy::render::RenderSet::PrepareBindGroups,
                    ),
                    bevy::prelude::not(
                        bevy::prelude::resource_exists::<GpuBufferBindGroup>,
                    ),
                ),
            );
        render_app
            .world_mut()
            .resource_mut::<bevy::render::render_graph::RenderGraph>()
            .add_node(ComputeNodeLabel, ComputeNode(ShaderStage::Loading, dispatches));
    }
}
fn create_setup(
    inits: Init,
) -> impl Fn(
    bevy::prelude::Commands,
    bevy::prelude::ResMut<
        bevy::asset::Assets<bevy::render::storage::ShaderStorageBuffer>,
    >,
    bevy::prelude::ResMut<bevy::asset::Assets<bevy::image::Image>>,
) {
    move |mut commands, mut buffers, mut images| {}
}
#[derive(bevy::prelude::Resource)]
struct GpuBufferBindGroup(bevy::render::render_resource::BindGroup);
fn prepare_bind_group(
    mut commands: bevy::prelude::Commands,
    pipeline: bevy::prelude::Res<ComputePipeline>,
    render_device: bevy::prelude::Res<bevy::render::renderer::RenderDevice>,
    buffers: bevy::prelude::Res<
        bevy::render::render_asset::RenderAssets<
            bevy::render::storage::GpuShaderStorageBuffer,
        >,
    >,
    images: bevy::prelude::Res<
        bevy::render::render_asset::RenderAssets<bevy::render::texture::GpuImage>,
    >,
) {
    let () = ();
    let bind_group = render_device
        .create_bind_group(
            None,
            &pipeline.layout,
            &bevy::render::render_resource::BindGroupEntries::sequential(()),
        );
    commands.insert_resource(GpuBufferBindGroup(bind_group));
}
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Entries {
    Main,
}
#[derive(Clone, Debug)]
pub struct ShaderDispatch {
    pub entry: Entries,
    pub workgroup: (u32, u32, u32),
}
#[derive(Clone)]
pub struct ShaderDispatches {
    pub on_startup: Vec<ShaderDispatch>,
    pub on_update: Vec<ShaderDispatch>,
}
#[derive(bevy::prelude::Resource, Debug)]
struct ComputePipeline {
    layout: bevy::render::render_resource::BindGroupLayout,
    pipelines: bevy::utils::HashMap<
        Entries,
        bevy::render::render_resource::CachedComputePipelineId,
    >,
}
impl bevy::prelude::FromWorld for ComputePipeline {
    fn from_world(world: &mut bevy::prelude::World) -> Self {
        let render_device = world.resource::<bevy::render::renderer::RenderDevice>();
        let layout = render_device
            .create_bind_group_layout(
                None,
                &bevy::render::render_resource::BindGroupLayoutEntries::sequential(
                    bevy::render::render_resource::ShaderStages::COMPUTE,
                    (),
                ),
            );
        let shader = bevy::asset::DirectAssetAccessExt::load_asset(world, "hello.wgsl");
        let pipeline_cache = world
            .resource::<bevy::render::render_resource::PipelineCache>();
        let pipelines = bevy::utils::HashMap::from([
            (
                Entries::Main,
                pipeline_cache
                    .queue_compute_pipeline(bevy::render::render_resource::ComputePipelineDescriptor {
                        label: None,
                        layout: vec![layout.clone()],
                        push_constant_ranges: Vec::new(),
                        shader: shader.clone(),
                        shader_defs: Vec::new(),
                        entry_point: "main".into(),
                        zero_initialize_workgroup_memory: false,
                    }),
            ),
        ]);
        Self { layout, pipelines }
    }
}
#[derive(Debug, Hash, PartialEq, Eq, Clone, bevy::render::render_graph::RenderLabel)]
struct ComputeNodeLabel;
struct ComputeNode(ShaderStage, ShaderDispatches);
impl bevy::render::render_graph::Node for ComputeNode {
    fn update(&mut self, world: &mut bevy::prelude::World) {
        let pipeline_cache = world
            .resource::<bevy::render::render_resource::PipelineCache>();
        let pipeline = world.resource::<ComputePipeline>();
        match self.0 {
            ShaderStage::Loading => {
                if self
                    .1
                    .on_startup
                    .iter()
                    .map(|dispatcher| {
                        pipeline_cache
                            .get_compute_pipeline_state(
                                *pipeline.pipelines.get(&dispatcher.entry).unwrap(),
                            )
                    })
                    .all(|state| match state {
                        bevy::render::render_resource::CachedPipelineState::Ok(_) => true,
                        bevy::render::render_resource::CachedPipelineState::Err(e) => {
                            panic!("Initializing assets/hello.wgsl\n{e:?}")
                        }
                        _ => false,
                    })
                {
                    self.0 = ShaderStage::Startup;
                }
            }
            ShaderStage::Startup => {
                if self
                    .1
                    .on_update
                    .iter()
                    .map(|dispatcher| {
                        pipeline_cache
                            .get_compute_pipeline_state(
                                *pipeline
                                    .pipelines
                                    .get(&dispatcher.entry)
                                    .expect("Pipeline entry not found"),
                            )
                    })
                    .all(|state| {
                        if let bevy::render::render_resource::CachedPipelineState::Ok(
                            _,
                        ) = state {
                            true
                        } else {
                            false
                        }
                    })
                {
                    self.0 = ShaderStage::Update;
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
        let pipeline_cache = world
            .resource::<bevy::render::render_resource::PipelineCache>();
        let pipeline = world.resource::<ComputePipeline>();
        let bind_group = world.resource::<GpuBufferBindGroup>();
        let mut pass = render_context
            .command_encoder()
            .begin_compute_pass(
                &bevy::render::render_resource::ComputePassDescriptor {
                    label: None,
                    ..bevy::utils::default()
                },
            );
        match self.0 {
            ShaderStage::Startup => {
                for dispatch in self.1.on_startup.iter() {
                    let pipeline = pipeline
                        .pipelines
                        .get(&dispatch.entry)
                        .expect("Pipeline entry not found");
                    if let Some(pipeline) = pipeline_cache
                        .get_compute_pipeline(*pipeline)
                    {
                        pass.set_bind_group(0, &bind_group.0, &[]);
                        pass.set_pipeline(pipeline);
                        let workgroup = dispatch.workgroup;
                        pass.dispatch_workgroups(workgroup.0, workgroup.1, workgroup.2);
                    }
                }
            }
            ShaderStage::Update => {
                for dispatch in self.1.on_update.iter() {
                    let pipeline = pipeline
                        .pipelines
                        .get(&dispatch.entry)
                        .expect("Pipeline entry not found");
                    if let Some(pipeline) = pipeline_cache
                        .get_compute_pipeline(*pipeline)
                    {
                        pass.set_bind_group(0, &bind_group.0, &[]);
                        pass.set_pipeline(pipeline);
                        let workgroup = dispatch.workgroup;
                        pass.dispatch_workgroups(workgroup.0, workgroup.1, workgroup.2);
                    }
                }
            }
            _ => {}
        }
        Ok(())
    }
}
#[derive(Clone)]
pub struct Init {}
