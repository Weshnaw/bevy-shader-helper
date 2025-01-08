use bevy_shazzy::internals::prelude::*;

pub type HelloShaderPlugin = ShaderPlugin<HelloBuffers, 4, 2>;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum HelloEntries {
    Main,
    Update,
}

#[derive(ShaderType, Clone, Default)]
pub struct Foo {
    pub bar: u32,
    pub bazz: f32,
}

#[derive(BufferGroup, ExtractResource, Clone)]
#[entry("main")]
#[entry("update")]
pub struct HelloBuffers {
    #[writeable]
    #[shader_type(Vec<u32>)]
    pub a: ReadWriteBuffer<ShaderStorageBuffer>,
    #[shader_type(Foo)]
    pub b: ReadBuffer<ShaderStorageBuffer>,
    #[shader_type(Vec3)]
    pub c: ReadBuffer<ShaderStorageBuffer>,
    #[writeable]
    #[texture(R32Float, D2)]
    pub d: ReadWriteBuffer<Image>,
}
