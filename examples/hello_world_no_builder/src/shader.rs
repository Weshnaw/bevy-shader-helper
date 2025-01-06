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

#[derive(Resource, ExtractResource, Clone, BufferGroup)]
#[data(HelloData)]
pub struct HelloBuffers {
    #[writeable]
    pub a: ReadWriteBuffer<ShaderStorageBuffer>,
    pub b: ReadBuffer<ShaderStorageBuffer>,
    pub c: ReadBuffer<ShaderStorageBuffer>,
    #[writeable]
    #[texture]
    pub d: ReadWriteBuffer<Image>,
}
