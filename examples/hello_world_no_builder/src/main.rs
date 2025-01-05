use bevy::{
    math::vec3,
    prelude::*,
    render::{gpu_readback::ReadbackComplete, render_resource::Extent3d},
};
use bevy_shader_helper::prelude::*;
use shader::{Foo, HelloBuffers, HelloData, HelloEntries, HelloShaderPlugin};

mod shader;

fn main() {
    let shader = HelloShaderPlugin::builder()
        .initial_data(HelloData {
            a: vec![1, 2, 3],
            b: Foo { bar: 1, bazz: 2. },
            c: vec3(1., 2., 3.),
            d: Extent3d {
                width: 3,
                height: 1,
                depth_or_array_layers: 1,
            }
            .into(),
        })
        .on_startup([(HelloEntries::Main, (3, 1, 1)).into()])
        .on_update([(HelloEntries::Update, (3, 1, 1)).into()])
        .build();

    App::new()
        .add_plugins((DefaultPlugins, shader))
        .add_systems(PostStartup, setup_readers)
        .run();
}

fn setup_readers(mut commands: Commands, buffers: Res<HelloBuffers>) {
    commands
        .spawn(buffers.a.readback())
        .observe(|t: Trigger<ReadbackComplete>| {
            let data: Vec<u32> = t.event().to_shader_type();
            info!(?data);
        });
    commands
        .spawn(buffers.d.readback())
        .observe(|t: Trigger<ReadbackComplete>| {
            let data: Vec<f32> = t.event().to_shader_type();
            info!(?data);
        });
}
