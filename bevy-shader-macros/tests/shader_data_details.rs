use bevy_shader_helper::{bevy::render::render_resource::ShaderStages, internals::prelude::ShaderDataDetails};

#[test]
fn test_macro() {
    #[derive(Clone, ShaderDataDetails)]
    pub struct HelloData {
        pub _a: Vec<u32>,
        pub _b: u32,
    }

    let test = HelloData::buffer_entries(ShaderStages::COMPUTE);
    assert_eq!(2, test.len());
}

