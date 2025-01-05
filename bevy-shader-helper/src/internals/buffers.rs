use bevy::{
    app::App,
    asset::{Asset, Assets, Handle, RenderAssetUsages},
    image::Image,
    prelude::{Commands, ResMut},
    render::{
        extract_resource::{ExtractResource, ExtractResourcePlugin},
        gpu_readback::Readback,
        render_asset::RenderAssets,
        render_resource::{
            BindGroupEntries, BufferUsages, ShaderType, TextureDimension, TextureFormat,
            TextureUsages, encase::internal::WriteInto,
        },
        storage::{GpuShaderStorageBuffer, ShaderStorageBuffer},
        texture::GpuImage,
    },
};

use crate::{ImageBuilder, ImageData};

pub trait GroupedBuffers<DataTy: Clone, const B: usize> {
    fn label() -> Option<&'static str> {
        // TODO: make this the correct return type -> impl wgpu::Label<'a>
        None
    }

    fn create_resource_extractor_plugins(app: &mut App)
    where
        Self: Sized + ExtractResource,
    {
        app.add_plugins((ExtractResourcePlugin::<Self>::default(),));
    }

    fn get_bindings<'a>(
        &'a self,
        buffers: &'a RenderAssets<GpuShaderStorageBuffer>,
        images: &'a RenderAssets<GpuImage>,
    ) -> BindGroupEntries<'a, B>; // TODO: consider refactoring the buffer inserters

    fn insert_resources(
        commands: &mut Commands,
        buffers: &mut Assets<ShaderStorageBuffer>,
        images: &mut Assets<Image>,
        d: DataTy,
    );
}

pub fn create_storage_buffer<DataTy: ShaderType + WriteInto>(
    buffers: &mut Assets<ShaderStorageBuffer>,
    data: DataTy,
    writeable: bool,
) -> Handle<ShaderStorageBuffer> {
    let mut data = ShaderStorageBuffer::from(data);
    if writeable {
        data.buffer_description.usage |= BufferUsages::COPY_SRC;
    }
    buffers.add(data)
}

pub fn create_texture_buffer(
    images: &mut Assets<Image>,
    builder: ImageBuilder,
    format: TextureFormat,
    dimension: TextureDimension,
    writeable: bool,
) -> Handle<Image> {
    let asset_usage = RenderAssetUsages::RENDER_WORLD;

    let mut image = match builder.data {
        ImageData::Fill(data) => {
            bevy::image::Image::new_fill(builder.size, dimension, &data, format, asset_usage)
        }
        ImageData::Data(vec) => {
            bevy::image::Image::new(builder.size, dimension, vec, format, asset_usage)
        }
        ImageData::Zeros => {
            let size = builder.size;
            let total = size.height * size.width * size.depth_or_array_layers;
            let total = total * format.block_copy_size(None).unwrap_or(0);
            // debug!("Creating image of {total} size");
            bevy::image::Image::new(
                size,
                dimension,
                vec![0; total as usize],
                format,
                asset_usage,
            )
        }
    };
    image.texture_descriptor.usage |= TextureUsages::STORAGE_BINDING;
    if writeable {
        image.texture_descriptor.usage |= TextureUsages::COPY_SRC;
    }
    images.add(image)
}

// NOTE:
// The structs are GPU land Read/Write terms so:
// WriteBuffer -> CPU Readable  meaning we want a readback
// ReadBuffer  -> CPU Writeable meaning we want to potentially be able to modify the buffer,
//                but we do not care about the readback because we already know it (unless of course it's ReadWrite)
pub struct WriteBuffer<T: Asset> {
    pub data: Handle<T>,
}
impl<T: Asset> Clone for WriteBuffer<T> {
    fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
        }
    }
}
impl<T: Asset> From<Handle<T>> for WriteBuffer<T> {
    fn from(data: Handle<T>) -> Self {
        Self { data }
    }
}
impl ReadableBuffer for WriteBuffer<ShaderStorageBuffer> {
    fn readback(&self) -> Readback {
        Readback::Buffer(self.data.clone())
    }
}

pub struct ReadBuffer<T: Asset> {
    pub data: Handle<T>,
}
impl<T: Asset> Clone for ReadBuffer<T> {
    fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
        }
    }
}
impl<T: Asset> From<Handle<T>> for ReadBuffer<T> {
    fn from(data: Handle<T>) -> Self {
        Self { data }
    }
}
impl WriteableBuffer for ReadBuffer<ShaderStorageBuffer> {
    type T = ShaderStorageBuffer;

    fn get_mut<'a>(&'a self, _buffers: &'a mut ResMut<Assets<Self::T>>) -> &'a mut Self::T
    where
        Self::T: Asset,
    {
        let _ = self.data;
        todo!()
    }
}

pub struct ReadWriteBuffer<T: Asset> {
    pub data: Handle<T>,
}
impl<T: Asset> Clone for ReadWriteBuffer<T> {
    fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
        }
    }
}
impl<T: Asset> From<Handle<T>> for ReadWriteBuffer<T> {
    fn from(data: Handle<T>) -> Self {
        Self { data }
    }
}
impl ReadableBuffer for ReadWriteBuffer<ShaderStorageBuffer> {
    fn readback(&self) -> Readback {
        Readback::Buffer(self.data.clone())
    }
}
impl ReadableBuffer for ReadWriteBuffer<Image> {
    fn readback(&self) -> Readback {
        Readback::Texture(self.data.clone())
    }
}

// The traits are CPUT land Read/Write terms so:
// Readable  -> GPU Wrote some data that I want to read via readback
// Writeable -> GPU wants to read some data from the buffer
pub trait ReadableBuffer {
    fn readback(&self) -> Readback;
}
pub trait WriteableBuffer {
    type T;
    fn get_mut<'a>(&'a self, buffers: &'a mut ResMut<Assets<Self::T>>) -> &'a mut Self::T
    where
        Self::T: Asset;
}
