use bevy::{
    prelude::*,
    render::{
        render_asset::RenderAssets,
        render_resource::{
            BindGroupEntries, BindGroupLayout, BindGroupLayoutEntries, CachedComputePipelineId,
            PipelineCache, ShaderStages, StorageTextureAccess, TextureFormat,
            binding_types::{storage_buffer, storage_buffer_read_only, texture_storage_2d},
        },
        storage::GpuShaderStorageBuffer,
        texture::GpuImage,
    },
};
use bevy_shazzy::internals::prelude::*;

pub type HelloShaderPlugin = ShaderPlugin<HelloBuffers, 4, 2>;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum HelloEntries {
    Main,
    Update,
}

#[derive(Clone, Default)]
pub struct HelloInitializer {
    pub a: Vec<u32>,
    pub b: Foo,
    pub c: Vec3,
    pub d: ImageBuilder<R32Float, D2>,
}

#[derive(Clone, ShaderType, Default)]
pub struct Foo {
    pub bar: u32,
    pub bazz: f32,
}

#[derive(Resource, ExtractResource, Clone)]
pub struct HelloBuffers {
    pub a: ReadWriteBuffer<ShaderStorageBuffer>,
    pub b: ReadBuffer<ShaderStorageBuffer>,
    pub c: ReadBuffer<ShaderStorageBuffer>,
    pub d: ReadWriteBuffer<Image>,
}

impl BufferGroup<4, 2> for HelloBuffers {
    type Initializer = HelloInitializer;

    fn get_bindings<'a>(
        &'a self,
        buffers: &'a RenderAssets<GpuShaderStorageBuffer>,
        images: &'a RenderAssets<GpuImage>,
    ) -> BindGroupEntries<'a, 4> {
        BindGroupEntries::sequential((
            self.a.binding(buffers),
            self.b.binding(buffers),
            self.c.binding(buffers),
            self.d.binding(images),
        ))
    }

    fn insert_resources(
        commands: &mut Commands,
        buffers: &mut Assets<ShaderStorageBuffer>,
        images: &mut Assets<Image>,
        d: Self::Initializer,
    ) {
        commands.insert_resource(Self {
            a: create_storage_buffer(buffers, d.a, true).into(),
            b: create_storage_buffer(buffers, d.b, true).into(),
            c: create_storage_buffer(buffers, d.c, true).into(),
            d: create_texture_buffer(images, d.d, true).into(),
        });
    }

    fn buffer_entries(stage: ShaderStages) -> BindGroupLayoutEntries<4> {
        BindGroupLayoutEntries::sequential(
            stage,
            (
                storage_buffer::<Vec<u32>>(false),
                storage_buffer_read_only::<Foo>(false),
                storage_buffer_read_only::<Vec3>(false),
                texture_storage_2d(TextureFormat::R32Float, StorageTextureAccess::ReadWrite),
            ),
        )
    }

    fn entries(
        pipeline_cache: &PipelineCache,
        layout: BindGroupLayout,
        shader: Handle<Shader>,
    ) -> [CachedComputePipelineId; 2] {
        [
            Self::create_entry(pipeline_cache, layout.clone(), shader.clone(), "main", None),
            Self::create_entry(
                pipeline_cache,
                layout.clone(),
                shader.clone(),
                "update",
                None,
            ),
        ]
    }
}

impl HelloBuffers {
    pub fn init(a: Vec<u32>, b: Foo, c: Vec3, d: ImageBuilder<R32Float, D2>) -> HelloInitializer {
        HelloInitializer { a, b, c, d }
    }
}
