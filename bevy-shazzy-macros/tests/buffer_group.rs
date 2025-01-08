use bevy_shazzy::{
    bevy::{Image, render::storage::ShaderStorageBuffer},
    internals::prelude::{BufferGroup, ReadBuffer, ReadWriteBuffer, Vec3},
};

#[test]
fn test_buffer_macro() {
    #[derive(BufferGroup)]
    #[entry("main")]
    #[entry("update")]
    pub struct HelloBuffers {
        #[writeable]
        #[shader_type(Vec<u32>)]
        pub a: ReadWriteBuffer<ShaderStorageBuffer>,
        #[shader_type(Vec3)]
        pub c: ReadBuffer<ShaderStorageBuffer>,
        #[writeable]
        #[texture(R32Float, D2)]
        pub d: ReadWriteBuffer<Image>,
    }
}
