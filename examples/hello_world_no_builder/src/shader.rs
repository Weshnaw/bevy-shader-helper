use bevy::{
    prelude::*,
    render::{
        extract_resource::ExtractResource,
        render_asset::RenderAssets,
        render_resource::{BindGroupEntries, IntoBinding, ShaderType},
        storage::{GpuShaderStorageBuffer, ShaderStorageBuffer},
        texture::GpuImage,
    },
};

use bevy_shader_helper::internals::prelude::*;

pub type HelloShaderPlugin = ShaderPlugin<HelloData, HelloEntries, HelloBuffers, 4, 2>;

#[derive(ShaderEntry, Debug, PartialEq, Eq, Hash, Clone)]
pub enum HelloEntries {
    Main,
    Update,
}

#[derive(ShaderType, Clone)]
pub struct Foo {
    pub bar: u32,
    pub bazz: f32,
}

#[derive(ShaderDataDetails, Clone)]
#[entry("main")]
#[entry("update")]
pub struct HelloData {
    pub a: Vec<u32>,
    #[read_only]
    pub b: Foo,
    #[read_only]
    pub c: Vec3,
    #[texture(ReadWrite, R32Float, D2)]
    pub d: ImageBuilder<R32Float, D2>,
}

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
                .get(&self.a.handle)
                .unwrap()
                .buffer
                .as_entire_buffer_binding(),
            buffers
                .get(&self.b.handle)
                .unwrap()
                .buffer
                .as_entire_buffer_binding(),
            buffers
                .get(&self.c.handle)
                .unwrap()
                .buffer
                .as_entire_buffer_binding(),
            images
                .get(&self.d.handle)
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
            a: create_storage_buffer(buffers, d.a, true).into(),
            b: create_storage_buffer(buffers, d.b, false).into(),
            c: create_storage_buffer(buffers, d.c, false).into(),
            d: create_texture_buffer(images, d.d, true).into(),
        });
    }
}
