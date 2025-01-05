use std::marker::PhantomData;

use bevy::render::render_resource::Extent3d;

use crate::internals::entries::Dispatch;

#[derive(Clone)]
pub enum ImageData {
    Fill([u8; 4]),
    Data(Vec<u8>),
    Zeros,
}

#[derive(Clone)]
pub struct ImageBuilder {
    pub size: Extent3d,
    pub data: ImageData,
}

pub struct ShaderBuilder<T: ?Sized, DataTy, EntriesTy> {
    pub(crate) initial_data: Option<DataTy>,
    pub(crate) dispatches: Option<Dispatch<EntriesTy>>,
    _phantom: PhantomData<T>,
}

impl<DataTy, EntriesTy, T: BuildableShader<DataTy, EntriesTy>> Default
    for ShaderBuilder<T, DataTy, EntriesTy>
{
    fn default() -> Self {
        Self {
            initial_data: Default::default(),
            dispatches: Default::default(),
            _phantom: Default::default(),
        }
    }
}

impl<DataTy, EntriesTy, T: BuildableShader<DataTy, EntriesTy>> ShaderBuilder<T, DataTy, EntriesTy> {
    pub fn initial_data(self, data: DataTy) -> Self {
        Self {
            initial_data: Some(data),
            dispatches: self.dispatches,
            _phantom: self._phantom,
        }
    }

    pub fn dispatches(self, dispatches: Dispatch<EntriesTy>) -> Self {
        Self {
            initial_data: self.initial_data,
            dispatches: Some(dispatches),
            _phantom: self._phantom,
        }
    }

    pub fn build(self) -> T {
        T::from_builder(self)
    }
}

pub trait BuildableShader<DataTy, EntriesTy> {
    fn from_builder(builder: ShaderBuilder<Self, DataTy, EntriesTy>) -> Self;
}
