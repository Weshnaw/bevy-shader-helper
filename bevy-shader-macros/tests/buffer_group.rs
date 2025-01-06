use bevy_shader_helper::{
    bevy::{Image, render::storage::ShaderStorageBuffer, Resource},
    internals::prelude::{BufferGroup, ReadBuffer, ReadWriteBuffer},
};

#[test]
fn test_buffer_macro() {
    #[derive(Clone)]
    struct HelloData {
        a: u32,
        b: u32,
        c: u32,
        d: Image,
    }

    #[derive(Resource, BufferGroup)]
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
}

// TODO: I don't fully understand why this does not work
// #[test]
// fn test_buffer_macro_no_idents() {
//     #[derive(Clone)]
//     struct HelloData(u32);
//     #[derive(Resource, BufferGroup)]
//     #[data(HelloData)]
//     pub struct HelloBuffers(#[writeable] ReadWriteBuffer<ShaderStorageBuffer>);
// }
