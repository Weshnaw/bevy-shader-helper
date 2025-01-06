use bevy_app::App;
use bevy_asset::{Asset, Assets, Handle};
use bevy_ecs::system::{Commands, ResMut};
use bevy_image::Image;
use bevy_render::{
    extract_resource::{ExtractResource, ExtractResourcePlugin},
    gpu_readback::Readback,
    render_asset::RenderAssets,
    render_resource::{
        BindGroupEntries, BindingResource, BufferUsages, IntoBinding, ShaderType, TextureUsages,
        encase::internal::WriteInto,
    },
    storage::{GpuShaderStorageBuffer, ShaderStorageBuffer},
    texture::GpuImage,
};

pub use bevy_shader_macros::BufferGroup;
pub trait BufferGroup<DataTy: Clone, const B: usize> {
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

pub trait HandleIntoBinding {
    type T;
    fn binding<'b>(&self, assets: &'b Self::T) -> BindingResource<'b>;
}

// Storage Buffers
impl HandleIntoBinding for ReadBuffer<ShaderStorageBuffer> {
    type T = RenderAssets<GpuShaderStorageBuffer>;
    fn binding<'b>(&self, assets: &'b Self::T) -> BindingResource<'b> {
        assets
            .get(&self.handle)
            .expect("Missing GPU Storage Buffer")
            .buffer
            .as_entire_binding()
    }
}
impl HandleIntoBinding for WriteBuffer<ShaderStorageBuffer> {
    type T = RenderAssets<GpuShaderStorageBuffer>;
    fn binding<'b>(&self, assets: &'b Self::T) -> BindingResource<'b> {
        assets
            .get(&self.handle)
            .expect("Missing GPU Storage Buffer")
            .buffer
            .as_entire_binding()
    }
}

impl HandleIntoBinding for ReadWriteBuffer<ShaderStorageBuffer> {
    type T = RenderAssets<GpuShaderStorageBuffer>;
    fn binding<'b>(&self, assets: &'b Self::T) -> BindingResource<'b> {
        assets
            .get(&self.handle)
            .expect("Missing GPU Storage Buffer")
            .buffer
            .as_entire_binding()
    }
}
// Texture Buffers
impl HandleIntoBinding for ReadBuffer<Image> {
    type T = RenderAssets<GpuImage>;
    fn binding<'b>(&self, assets: &'b Self::T) -> BindingResource<'b> {
        assets
            .get(&self.handle)
            .expect("Missing GPU Image")
            .texture_view
            .into_binding()
    }
}
impl HandleIntoBinding for WriteBuffer<Image> {
    type T = RenderAssets<GpuImage>;
    fn binding<'b>(&self, assets: &'b Self::T) -> BindingResource<'b> {
        assets
            .get(&self.handle)
            .expect("Missing GPU Image")
            .texture_view
            .into_binding()
    }
}

impl HandleIntoBinding for ReadWriteBuffer<Image> {
    type T = RenderAssets<GpuImage>;
    fn binding<'b>(&self, assets: &'b Self::T) -> BindingResource<'b> {
        assets
            .get(&self.handle)
            .expect("Missing GPU Image")
            .texture_view
            .into_binding()
    }
}
