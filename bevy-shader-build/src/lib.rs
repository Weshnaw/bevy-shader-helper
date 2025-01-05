mod internals;

use std::{
    fs::{self, File},
    io::Write,
    path::PathBuf,
};

use internals::EntryError;
use naga::front::wgsl;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ShaderError {
    #[error("No File Name found")]
    NoFileName,
    #[error("Failed to read shader file")]
    FailedToReadFile(#[from] std::io::Error),
    #[error("Failed to parse wgsl shader file")]
    WGSLParseError(#[from] wgsl::ParseError),
    #[error("Failed to parse shader entries")]
    EntryError(#[from] EntryError),
}

pub(crate) type Result<T> = std::result::Result<T, ShaderError>;

pub fn compile_shader<T: AsRef<str>>(name: T, shader: impl Into<PathBuf>) -> Result<()> {
    let file: PathBuf = shader.into();
    let path = file.clone();
    let path = path.as_os_str().to_string_lossy();
    let mut assets_file = PathBuf::from("assets");
    assets_file.push(file);
    let rust_file = internals::compile_shader(name, fs::read_to_string(assets_file)?, path)?;

    let mut file = File::create("../examples/generated_temp.rs")?;
    file.write_all(rust_file.as_bytes())?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use tracing::debug;

    use super::*;

    #[test_log::test]
    fn test_invalid_shader_path() {
        let res = compile_shader("test", "/../foo");

        assert!(res.is_err());
    }

    #[test_log::test]
    fn test_shader_example() {
        let res = compile_shader("hello", "hello.wgsl");
        debug!(?res);
        assert!(res.is_err());
    }
}
