use bevy::{
    prelude::*,
    render::{
        extract_resource::{ExtractResource, ExtractResourcePlugin},
        render_asset::RenderAssets,
        render_graph::RenderLabel,
        render_resource::{
            BindGroupEntries, BindGroupLayout, BindGroupLayoutEntries, CachedComputePipelineId,
            IntoBinding, PipelineCache, ShaderStages, ShaderType, StorageTextureAccess,
            TextureDimension, TextureFormat,
            binding_types::{storage_buffer, storage_buffer_read_only, texture_storage_2d},
        },
        storage::{GpuShaderStorageBuffer, ShaderStorageBuffer},
        texture::GpuImage,
    },
};

use bevy_shader_helper::internals::prelude::*;

pub type HelloShaderPlugin =
    ShaderPlugin<HelloData, HelloEntries, HelloBuffers, HelloComputePipeline, HelloShader, 4>;
pub type HelloComputePipeline = ComputePipeline<4, 2, HelloData>;

#[derive(Debug, Hash, PartialEq, Eq, Clone, RenderLabel)]
pub struct HelloShader;

#[derive(Debug, PartialEq, Eq, Hash, Clone, ShaderEntry)]
pub enum HelloEntries {
    Main,
    Update,
}

#[derive(Clone, ShaderType)]
pub struct Foo {
    pub bar: u32,
    pub bazz: f32,
}

#[derive(Clone)]
pub struct HelloData {
    pub a: Vec<u32>,
    pub b: Foo,
    pub c: Vec3,
    pub d: ImageBuilder,
}

impl ShaderDetails<4, 2> for HelloData {
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

// I don't like this but I do not know how to improve it
#[derive(Resource, ExtractResource, Clone)]
pub struct HelloBuffers {
    pub a: ReadWriteBuffer<ShaderStorageBuffer>,
    pub b: ReadBuffer<ShaderStorageBuffer>,
    pub c: ReadBuffer<ShaderStorageBuffer>,
    pub d: ReadWriteBuffer<Image>,
}

impl GroupedBuffers<HelloData, 4> for HelloBuffers {
    fn get_bindings<'a>(
        &'a self,
        buffers: &'a RenderAssets<GpuShaderStorageBuffer>,
        images: &'a RenderAssets<GpuImage>,
    ) -> BindGroupEntries<'a, 4> {
        BindGroupEntries::sequential((
            buffers
                .get(&self.a.data)
                .unwrap()
                .buffer
                .as_entire_buffer_binding(),
            buffers
                .get(&self.b.data)
                .unwrap()
                .buffer
                .as_entire_buffer_binding(),
            buffers
                .get(&self.c.data)
                .unwrap()
                .buffer
                .as_entire_buffer_binding(),
            images
                .get(&self.d.data)
                .unwrap()
                .texture_view
                .into_binding(),
        ))
    }

    fn insert_resources(
        commands: &mut Commands,
        buffers: &mut Assets<ShaderStorageBuffer>,
        images: &mut Assets<Image>,
        d: HelloData,
    ) {
        commands.insert_resource(Self {
            a: create_storage_buffer(buffers, d.a).into(),
            b: create_storage_buffer(buffers, d.b).into(),
            c: create_storage_buffer(buffers, d.c).into(),
            d: create_texture_buffer(images, d.d, TextureFormat::R32Float, TextureDimension::D2)
                .into(),
        });
    }

    fn create_resource_extractor_plugins(app: &mut App) {
        app.add_plugins((ExtractResourcePlugin::<Self>::default(),));
    }
}
