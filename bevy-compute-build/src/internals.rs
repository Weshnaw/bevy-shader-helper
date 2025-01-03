use liquid::{Object, model};
use naga::{EntryPoint, ShaderStage, front::wgsl};
use thiserror::Error;
use tracing::debug;
// use wgpu_types::StorageTextureAccess;

use crate::{Result, template_filters::*};

const COMPUTE: &str = include_str!("../templates/compute_shader.rs.liquid");

pub(crate) fn compile_shader<T1: AsRef<str>, T2: AsRef<str>, T3: AsRef<str>>(
    name: T1,
    shader: T2,
    shader_path: T3,
) -> Result<String> {
    let shader = wgsl::parse_str(shader.as_ref())?;

    let template = liquid::ParserBuilder::with_stdlib()
        .filter(KebabCaseFilterParser)
        .filter(LowerCamelCaseFilterParser)
        .filter(PascalCaseFilterParser)
        .filter(ShoutyKebabCaseFilterParser)
        .filter(ShoutySnakeCaseFilterParser)
        .filter(SnakeCaseFilterParser)
        .filter(TitleCaseFilterParser)
        .filter(UpperCamelCaseFilterParser)
        .build()
        .expect("Failed to create liquid parser");

    let mut globals = liquid::object!({
        "shader_name": name.as_ref(),
        "shader_path": shader_path.as_ref(),
    });
    let stage = handle_entries(&shader.entry_points, &mut globals)?;

    let template = match stage {
        ShaderStage::Compute => template.parse(COMPUTE).expect("Failed to parse compute template"),
        _ => todo!("Handle other shader stages"),
    };

    debug!(?globals);
    let templated = template.render(&globals).unwrap();
    let tokens = syn::parse_str(&templated).expect("Failed to parse template");
    let formated = prettyplease::unparse(&tokens);
    Ok(formated)
}

#[derive(Error, Debug)]
pub enum EntryError {
    #[error("Shader has multiple stages")]
    MultipleStages, // For now if a shader has multi-stages in one file this will fail
    #[error("Shader has no stages")]
    NoStages,
}

// TODO: better error return, i'm just being lazy
fn handle_entries(entries: &Vec<EntryPoint>, globals: &mut Object) -> Result<ShaderStage> {
    let mut stage = None;

    let mut ar = vec![];
    for entry in entries {
        if let Some(stage) = stage {
            if stage != entry.stage {
                return Err(EntryError::MultipleStages.into());
            }
        } else {
            stage = Some(entry.stage);
        }
        ar.push(model::value!(entry.name));
    }
    globals.insert("entries".into(), ar.into());

    stage.ok_or(EntryError::NoStages.into())
}

// TODO: Handle several buffer types
// wgsl                     -> bevy_render
// array<T>                 -> BufferVec<T>
// uniform<T>               -> UniformBuffer<T>
// texture_storage_Nd<T, _> -> Texture
// _                        -> StorageBuffer<T>
// Extra note, we can map the types:
// vecN<f32> -> VecN
// vecN<u32> -> UVecN
// ...
// TODO: Handle structs
// TODO: Handle WebGL2 cases where storage buffers can't be used
//       for example using GpuArrayBuffer<T> instead of BufferVec
// TODO: Handle cases where RawBufferVec can be used at no issue
// TODO: Figure out when dynamic buffers need to be used

// enum DataType {
//     F32,
//     U32,
//     Struct, // TODO more
// }

// enum BufferType {
//     Uniform(DataType),
//     Array(DataType),
//     Texture(DataType, StorageTextureAccess),
//     Storage(DataType),
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn test_invalid_shader() {
        let res = compile_shader("test", "foo", "");

        assert!(res.is_err())
    }

    #[test_log::test]
    fn test_entry_points() {
        let res = compile_shader("test", r#"@compute @workgroup_size(1) fn main() {}"#, "");

        assert!(res.is_ok())
    }

    //     #[test_log::test]
    //     fn test_buffers() {
    //         compile_shader(
    //             r#"
    // struct Foo {
    //   bar: u32,
    //   baz: f32
    // }
    // @group(0) @binding(0) var<storage, read_write> a: array<u32>;
    // @group(0) @binding(0) var<storage, read> b: Foo;
    // @group(0) @binding(0) var<uniform> c: u32;
    // @group(0) @binding(0) var d: texture_storage_2d<r32float, write>;

    // @compute @workgroup_size(1)
    // fn main() {}
    // "#,
    //         );
    //     }
}
