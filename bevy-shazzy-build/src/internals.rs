#![allow(unused_variables, dead_code)]
use naga::{
    Arena, EntryPoint, GlobalVariable, Handle, ImageClass, ImageDimension, Scalar, StructMember,
    Type, TypeInner, UniqueArena, front::wgsl,
};
use thiserror::Error;
use tracing::{debug, info};
use wgpu_types::ShaderStages;

pub(crate) fn compile_shader<T1: AsRef<str>, T2: AsRef<str>, T3: AsRef<str>>(
    name: T1,
    shader: T2,
    shader_path: T3,
) -> crate::Result<String> {
    let shader = wgsl::parse_str(shader.as_ref())?;

    debug!(?shader);

    todo!()
}

#[derive(Error, Debug)]
pub enum EntryError {
    #[error("Shader has multiple stages")]
    MultipleStages, // For now if a shader has multi-stages in one file this will fail
    #[error("Shader has no stages")]
    NoStages,
}

// TODO: better error return, i'm just being lazy
// TODO: return a bitflag result to allow for multiple shader stages to be acknowledged
fn handle_entries(entries: &[EntryPoint]) -> crate::Result<ShaderStages> {
    let _ = entries.iter().map(|entry| (&entry.name, entry.stage));

    todo!()
}

fn basic_type(scalar: &Scalar) -> u32 {
    info!(?scalar);

    scalar.width as u32
}

fn vector_type(scalar: &Scalar) -> u32 {
    info!(?scalar);

    scalar.width as u32
}

fn array_type(base: &Handle<Type>, types: &UniqueArena<Type>) -> u32 {
    let base = types
        .get_handle(*base)
        .expect("Failed to get shader array base data type");
    info!(?base);

    0
}

fn struct_type(members: &[StructMember], _types: &UniqueArena<Type>, span: &u32) -> u32 {
    let members: Vec<_> = members
        .iter()
        .map(|mem| {
            _types
                .get_handle(mem.ty)
                .expect("Failed to get shader struct member types")
        })
        .collect();
    info!(?members, ?span);

    *span
}

fn texture_type(dim: &ImageDimension, class: &ImageClass) -> u32 {
    info!(?dim, ?class);

    0
}

// TODO: fail if no buffers
// TODO: should probably use naga size hints rather then rust size_of::<T>()
// TODO: should handle pre-sized arrays, if they exist
fn handle_buffers(
    variables: &Arena<GlobalVariable>,
    types: &UniqueArena<Type>,
) -> crate::Result<()> {
    for v in variables.iter() {
        let ty = v.1.ty;
        let ty = types
            .get_handle(ty)
            .expect("Failed to get shader data type");
        let ty = &ty.inner;
        match ty {
            TypeInner::Scalar(scalar) => basic_type(scalar),
            TypeInner::Atomic(scalar) => basic_type(scalar),
            TypeInner::Vector { size: _, scalar } => vector_type(scalar),
            TypeInner::Array {
                base,
                size: _,
                stride: _,
            } => array_type(base, types),
            TypeInner::Struct { members, span } => struct_type(members, types, span),
            TypeInner::Image {
                dim,
                arrayed: _,
                class,
            } => texture_type(dim, class),
            _ => todo!("Unhandled buffer type"),
        };
    }

    Ok(())
}

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
}
