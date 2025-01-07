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
        if self.dispatches.on_startup.is_empty() && self.dispatches.on_update.is_empty() {
            return; 
        }
        app.add_plugins((
            
            bevy::render::extract_resource::ExtractResourcePlugin::<ABuffer>::default(),
            bevy::render::extract_resource::ExtractResourcePlugin::<BBuffer>::default(),
            bevy::render::extract_resource::ExtractResourcePlugin::<CBuffer>::default(),
            
            bevy::render::extract_resource::ExtractResourcePlugin::<DBuffer>::default(),
        ));

        app.add_systems(
            bevy::app::Startup,
            create_setup(self.initializations.clone()),
        );
    }

    fn finish(&self, app: &mut bevy::app::App) {
        let dispatches = self.dispatches.clone();
        if dispatches.on_startup.is_empty() && dispatches.on_update.is_empty() {
            return; 
        }
        let render_app = app.sub_app_mut(bevy::render::RenderApp);
        render_app
            .init_resource::<ComputePipeline>()
            
            .add_systems(
                bevy::render::Render,
                bevy::prelude::IntoSystemConfigs::run_if(
                    bevy::prelude::IntoSystemConfigs::in_set(
                        prepare_bind_group,
                        bevy::render::RenderSet::PrepareBindGroups,
                    ),
                    bevy::prelude::not(bevy::prelude::resource_exists::<GpuBufferBindGroup>),
                    
                ),
            );

        render_app
            .world_mut()
            .resource_mut::<bevy::render::render_graph::RenderGraph>()
            .add_node(
                ComputeNodeLabel,
                ComputeNode(ShaderStage::Loading, dispatches),
            );
        
    }
}

fn create_setup(
    inits: Init,
) -> impl Fn(
    bevy::prelude::Commands,
    
    bevy::prelude::ResMut<bevy::asset::Assets<bevy::render::storage::ShaderStorageBuffer>>,
    
    bevy::prelude::ResMut<bevy::asset::Assets<bevy::image::Image>>,
) {
    move |mut commands,
          
          mut buffers,
          
          mut images| {
        
        let mut a_buffer = bevy::render::storage::ShaderStorageBuffer::from(inits.a_data.clone());
        
        a_buffer.buffer_description.usage |= bevy::render::render_resource::BufferUsages::COPY_SRC;
        let a_buffer = buffers.add(a_buffer);
        commands.insert_resource(ABuffer(a_buffer));

        let b_buffer = bevy::render::storage::ShaderStorageBuffer::from(inits.b_data.clone());
        
        let b_buffer = buffers.add(b_buffer);
        commands.insert_resource(BBuffer(b_buffer));

        let c_buffer = bevy::render::storage::ShaderStorageBuffer::from(inits.c_data.clone());
        let c_buffer = buffers.add(c_buffer);
        commands.insert_resource(CBuffer(c_buffer));

        

        let image_details = inits.d_image.clone();
        let format = bevy::render::render_resource::TextureFormat::R32Float;
        let dimension = bevy::render::render_resource::TextureDimension::D2;
        let asset_usage = bevy::asset::RenderAssetUsages::RENDER_WORLD;
        let mut d_buffer = match image_details.data {
            ImageData::Fill(data) => bevy::image::Image::new_fill(
                image_details.size,
                dimension,
                &data,
                format,
                asset_usage,
            ),
            ImageData::Data(vec) => {
                bevy::image::Image::new(image_details.size, dimension, vec, format, asset_usage)
            }
            ImageData::Zeros => {
                let size = image_details.size;
                let total = size.height * size.width * size.depth_or_array_layers;
                let total = total * format.block_copy_size(None).unwrap_or(0);
                bevy::image::Image::new(
                    size,
                    dimension,
                    vec![0; total as usize],
                    format,
                    asset_usage,
                )
            }
        };

        
        d_buffer.texture_descriptor.usage |= bevy::render::render_resource::TextureUsages::COPY_SRC
            | bevy::render::render_resource::TextureUsages::STORAGE_BINDING;
        let d_buffer = images.add(d_buffer);
        commands.insert_resource(DBuffer(d_buffer));
    }
}

#[derive(bevy::prelude::Resource)]
struct GpuBufferBindGroup(bevy::render::render_resource::BindGroup); 

fn prepare_bind_group(
    mut commands: bevy::prelude::Commands,
    pipeline: bevy::prelude::Res<ComputePipeline>, 
    render_device: bevy::prelude::Res<bevy::render::renderer::RenderDevice>,
    
    a_buffer: bevy::prelude::Res<ABuffer>,
    b_buffer: bevy::prelude::Res<BBuffer>,
    c_buffer: bevy::prelude::Res<CBuffer>,
    
    d_buffer: bevy::prelude::Res<DBuffer>,
    
    buffers: bevy::prelude::Res<
        bevy::render::render_asset::RenderAssets<bevy::render::storage::GpuShaderStorageBuffer>,
    >,
    
    images: bevy::prelude::Res<
        bevy::render::render_asset::RenderAssets<bevy::render::texture::GpuImage>,
    >,
) {
    
    let (
        
        a_buffer,
        b_buffer,
        c_buffer,
    ) = (
        
        buffers
            .get(&a_buffer.0)
            .expect("Failed to retrieve a_buffer"),
        buffers
            .get(&b_buffer.0)
            .expect("Failed to retrieve b_buffer"),
        buffers
            .get(&c_buffer.0)
            .expect("Failed to retrieve c_buffer"),
    );
    
    let (
        
        d_buffer,
    ) = (
        
        images
            .get(&d_buffer.0)
            .expect("Failed to retrieve c_buffer"),
    );

    let bind_group = render_device.create_bind_group(
        None, 
        &pipeline.layout,
        &bevy::render::render_resource::BindGroupEntries::sequential((
            
            a_buffer.buffer.as_entire_buffer_binding(),
            b_buffer.buffer.as_entire_buffer_binding(),
            c_buffer.buffer.as_entire_buffer_binding(),
            
            bevy::render::render_resource::IntoBinding::into_binding(&d_buffer.texture_view),
        )),
    );
    commands.insert_resource(GpuBufferBindGroup(bind_group)); 
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Entries {
    
    Main,
    Update,
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
    pipelines:
        bevy::utils::HashMap<Entries, bevy::render::render_resource::CachedComputePipelineId>,
}

impl bevy::prelude::FromWorld for ComputePipeline {
    
    fn from_world(world: &mut bevy::prelude::World) -> Self {
        let render_device = world.resource::<bevy::render::renderer::RenderDevice>();
        let layout = render_device.create_bind_group_layout(
            None, 
            &bevy::render::render_resource::BindGroupLayoutEntries::sequential(
                bevy::render::render_resource::ShaderStages::COMPUTE,
                (
                    
                    bevy::render::render_resource::binding_types::storage_buffer::<Vec<u32>>(false),
                    bevy::render::render_resource::binding_types::storage_buffer_read_only::<Foo>(false),
                    bevy::render::render_resource::binding_types::storage_buffer_read_only::<bevy::math::Vec3>(false),
                    
                    bevy::render::render_resource::IntoBindGroupLayoutEntryBuilder::into_bind_group_layout_entry_builder(bevy::render::render_resource::BindingType::StorageTexture {
                        access: bevy::render::render_resource::StorageTextureAccess::ReadWrite,
                        format: bevy::render::render_resource::TextureFormat::R32Float,
                        view_dimension: bevy::render::render_resource::TextureViewDimension::D2,
                    }),
                ),
            ),
        );
        let shader = bevy::asset::DirectAssetAccessExt::load_asset(world, "shaders/hello.wgsl"); 
        let pipeline_cache = world.resource::<bevy::render::render_resource::PipelineCache>();

        let pipelines = bevy::utils::HashMap::from([
            
            (
                Entries::Main,
                pipeline_cache.queue_compute_pipeline(
                    bevy::render::render_resource::ComputePipelineDescriptor {
                        label: None, 
                        layout: vec![layout.clone()],
                        push_constant_ranges: Vec::new(),
                        shader: shader.clone(),
                        shader_defs: Vec::new(),
                        entry_point: "main".into(),
                        zero_initialize_workgroup_memory: false,
                    },
                ),
            ),
            (
                Entries::Update,
                pipeline_cache.queue_compute_pipeline(
                    bevy::render::render_resource::ComputePipelineDescriptor {
                        label: None, 
                        layout: vec![layout.clone()],
                        push_constant_ranges: Vec::new(),
                        shader: shader.clone(),
                        shader_defs: Vec::new(),
                        entry_point: "update".into(),
                        zero_initialize_workgroup_memory: false,
                    },
                ),
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
        let pipeline_cache = world.resource::<bevy::render::render_resource::PipelineCache>();
        let pipeline = world.resource::<ComputePipeline>();

        match self.0 {
            ShaderStage::Loading => {
                if self
                    .1
                    .on_startup
                    .iter()
                    .map(|dispatcher| {
                        pipeline_cache.get_compute_pipeline_state(
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
                    self.0 = ShaderStage::Startup
                }
            }
            ShaderStage::Startup => {
                if self
                    .1
                    .on_update
                    .iter()
                    .map(|dispatcher| {
                        pipeline_cache.get_compute_pipeline_state(
                            *pipeline
                                .pipelines
                                .get(&dispatcher.entry)
                                .expect("Pipeline entry not found"),
                        )
                    })
                    .all(|state| {
                        if let bevy::render::render_resource::CachedPipelineState::Ok(_) = state {
                            true
                        } else {
                            false
                        }
                    })
                {
                    self.0 = ShaderStage::Update
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
        let pipeline_cache = world.resource::<bevy::render::render_resource::PipelineCache>();
        let pipeline = world.resource::<ComputePipeline>();
        
        let bind_group = world.resource::<GpuBufferBindGroup>();
        
        let mut pass = render_context.command_encoder().begin_compute_pass(
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
                    if let Some(pipeline) = pipeline_cache.get_compute_pipeline(*pipeline) {
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
                    if let Some(pipeline) = pipeline_cache.get_compute_pipeline(*pipeline) {
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
pub struct Init {
    
    pub a_data: Vec<u32>,
    pub b_data: Foo,
    pub c_data: bevy::math::Vec3,
    
    pub d_image: ImageInit,
}


#[derive(bevy::prelude::Resource, bevy::render::extract_resource::ExtractResource, Clone)]
pub struct ABuffer(bevy::asset::Handle<bevy::render::storage::ShaderStorageBuffer>);
impl ABuffer {
    
    pub fn spawn_readback<E: bevy::prelude::Event, B: bevy::prelude::Bundle, M>(
        &self,
        commands: &mut bevy::prelude::Commands,
        callback: impl bevy::ecs::system::IntoObserverSystem<E, B, M>,
    ) {
        commands
            .spawn(bevy::render::gpu_readback::Readback::buffer(self.0.clone()))
            .observe(callback);
    }
    
    pub fn set_data(
        &self,
        buffers: &mut bevy::prelude::ResMut<
            bevy::asset::Assets<bevy::render::storage::ShaderStorageBuffer>,
        >,
        val: Vec<u32>,
    ) {
        let buffer = buffers.get_mut(self.0.id()).unwrap();
        buffer.set_data(val);
    }
    pub fn modify(
        &self,
        buffers: &mut bevy::prelude::ResMut<
            bevy::asset::Assets<bevy::render::storage::ShaderStorageBuffer>,
        >,
        f: impl Fn(&mut bevy::render::storage::ShaderStorageBuffer),
    ) {
        let data = buffers.get_mut(self.0.id()).unwrap();
        f(data)
    }
}

#[derive(Clone, bevy::render::render_resource::ShaderType)]
pub struct Foo {
    pub bar: u32,
    pub baz: f32,
}

#[derive(bevy::prelude::Resource, bevy::render::extract_resource::ExtractResource, Clone)]
pub struct BBuffer(bevy::asset::Handle<bevy::render::storage::ShaderStorageBuffer>);
impl BBuffer {
    
    pub fn set_data(
        &self,
        buffers: &mut bevy::prelude::ResMut<
            bevy::asset::Assets<bevy::render::storage::ShaderStorageBuffer>,
        >,
        val: Foo,
    ) {
        let buffer = buffers.get_mut(self.0.id()).unwrap();
        buffer.set_data(val);
    }
    pub fn modify(
        &self,
        buffers: &mut bevy::prelude::ResMut<
            bevy::asset::Assets<bevy::render::storage::ShaderStorageBuffer>,
        >,
        f: impl Fn(&mut bevy::render::storage::ShaderStorageBuffer),
    ) {
        let data = buffers.get_mut(self.0.id()).unwrap();
        f(data)
    }
}

#[derive(bevy::prelude::Resource, bevy::render::extract_resource::ExtractResource, Clone)]
pub struct CBuffer(bevy::asset::Handle<bevy::render::storage::ShaderStorageBuffer>);
impl CBuffer {
    
    pub fn set_data(
        &self,
        buffers: &mut bevy::prelude::ResMut<
            bevy::asset::Assets<bevy::render::storage::ShaderStorageBuffer>,
        >,
        val: bevy::math::Vec3,
    ) {
        let buffer = buffers.get_mut(self.0.id()).unwrap();
        buffer.set_data(val);
    }
    pub fn modify(
        &self,
        buffers: &mut bevy::prelude::ResMut<
            bevy::asset::Assets<bevy::render::storage::ShaderStorageBuffer>,
        >,
        f: impl Fn(&mut bevy::render::storage::ShaderStorageBuffer),
    ) {
        let data = buffers.get_mut(self.0.id()).unwrap();
        f(data)
    }
}


#[derive(bevy::prelude::Resource, bevy::render::extract_resource::ExtractResource, Clone)]
pub struct DBuffer(bevy::asset::Handle<bevy::image::Image>);
impl DBuffer {
    
    pub fn spawn_readback<E: bevy::prelude::Event, B: bevy::prelude::Bundle, M>(
        &self,
        commands: &mut bevy::prelude::Commands,
        callback: impl bevy::ecs::system::IntoObserverSystem<E, B, M>,
    ) {
        commands
            .spawn(bevy::render::gpu_readback::Readback::texture(
                self.0.clone(),
            ))
            .observe(callback);
    }

    
    pub fn modify(
        &self,
        buffers: &mut bevy::prelude::ResMut<bevy::asset::Assets<bevy::image::Image>>,
        f: impl Fn(&mut bevy::image::Image),
    ) {
        let data = buffers.get_mut(self.0.id()).unwrap();
        f(data)
    }
}
