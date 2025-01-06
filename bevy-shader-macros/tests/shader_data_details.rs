use bevy_shader_helper::{
    bevy::render::render_resource, internals::prelude::ShaderDataDetails, texture_details::{R32Float, D2}, ImageBuilder
};

#[test]
fn test_data_macro() {
    #[derive(Clone, ShaderDataDetails)]
    #[entry("main")]
    #[entry("update", "label")]
    // #[entry]
    pub struct HelloData {
        pub _a: Vec<u32>,
        #[read_only]
        pub _b: u32,
        #[texture(ReadWrite, R32Float, D2)]
        pub _c: ImageBuilder<R32Float, D2>,
    }

    let bind_group = HelloData::buffer_entries(render_resource::ShaderStages::COMPUTE);
    assert_eq!(3, bind_group.len());
}
