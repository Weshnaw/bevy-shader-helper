use bevy::{
    app::App,
    asset::{Asset, Assets, Handle},
    image::Image,
    prelude::{Commands, ResMut},
    render::{
        extract_resource::{ExtractResource, ExtractResourcePlugin},
        gpu_readback::Readback,
        render_asset::RenderAssets,
        render_resource::{
            BindGroupEntries, BufferUsages, ShaderType,
            TextureUsages, encase::internal::WriteInto,
        },
        storage::{GpuShaderStorageBuffer, ShaderStorageBuffer},
        texture::GpuImage,
    },
};

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
    image: impl Into<Image>,
    writeable: bool,
) -> Handle<Image> {
    let mut image: Image = image.into();
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
    pub handle: Handle<T>,
}
impl<T: Asset> Clone for WriteBuffer<T> {
    fn clone(&self) -> Self {
        Self {
            handle: self.handle.clone(),
        }
    }
}
impl<T: Asset> From<Handle<T>> for WriteBuffer<T> {
    fn from(data: Handle<T>) -> Self {
        Self { handle: data }
    }
}
impl ReadableBuffer for WriteBuffer<ShaderStorageBuffer> {
    fn readback(&self) -> Readback {
        Readback::Buffer(self.handle.clone())
    }
}

pub struct ReadBuffer<T: Asset> {
    pub handle: Handle<T>,
}
impl<T: Asset> Clone for ReadBuffer<T> {
    fn clone(&self) -> Self {
        Self {
            handle: self.handle.clone(),
        }
    }
}
impl<T: Asset> From<Handle<T>> for ReadBuffer<T> {
    fn from(data: Handle<T>) -> Self {
        Self { handle: data }
    }
}
impl WriteableBuffer for ReadBuffer<ShaderStorageBuffer> {
    type T = ShaderStorageBuffer;

    fn get_mut<'a>(&'a self, _buffers: &'a mut ResMut<Assets<Self::T>>) -> &'a mut Self::T
    where
        Self::T: Asset,
    {
        let _ = self.handle;
        todo!()
    }
}

pub struct ReadWriteBuffer<T: Asset> {
    pub handle: Handle<T>,
}
impl<T: Asset> Clone for ReadWriteBuffer<T> {
    fn clone(&self) -> Self {
        Self {
            handle: self.handle.clone(),
        }
    }
}
impl<T: Asset> From<Handle<T>> for ReadWriteBuffer<T> {
    fn from(data: Handle<T>) -> Self {
        Self { handle: data }
    }
}
impl ReadableBuffer for ReadWriteBuffer<ShaderStorageBuffer> {
    fn readback(&self) -> Readback {
        Readback::Buffer(self.handle.clone())
    }
}
impl ReadableBuffer for ReadWriteBuffer<Image> {
    fn readback(&self) -> Readback {
        Readback::Texture(self.handle.clone())
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
