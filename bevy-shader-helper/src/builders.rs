use std::marker::PhantomData;

use bevy::render::render_resource::Extent3d;

use crate::internals::entries::{Dispatch, Entry};

#[derive(Clone, Default)]
pub enum ImageData {
    #[default]
    Zeros,
    Fill([u8; 4]),
    Data(Vec<u8>),
}

#[derive(Clone)]
pub struct ImageBuilder {
    pub size: Extent3d,
    pub data: ImageData,
}

impl From<Extent3d> for ImageBuilder {
    fn from(value: Extent3d) -> Self {
        Self {
            size: value,
            data: Default::default(),
        }
    }
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
    pub fn initial_data(mut self, data: DataTy) -> Self {
        self.initial_data = Some(data);

        self
    }

    pub fn on_startup<E: Into<Vec<Entry<EntriesTy>>>>(mut self, entries: E) -> Self {
        let dispatch = match self.dispatches {
            Some(mut dispatch) => {
                dispatch.on_startup = entries.into();
                dispatch
            }
            None => Dispatch {
                on_startup: entries.into(),
                on_update: vec![],
            },
        };

        self.dispatches = Some(dispatch);

        self
    }
    pub fn on_update<E: Into<Vec<Entry<EntriesTy>>>>(mut self, entries: E) -> Self {
        let dispatch = match self.dispatches {
            Some(mut dispatch) => {
                dispatch.on_update = entries.into();
                dispatch
            }
            None => Dispatch {
                on_startup: vec![],
                on_update: entries.into(),
            },
        };

        self.dispatches = Some(dispatch);

        self
    }

    pub fn build(self) -> T {
        T::from_builder(self)
    }
}

pub trait BuildableShader<DataTy, EntriesTy> {
    fn from_builder(builder: ShaderBuilder<Self, DataTy, EntriesTy>) -> Self;
    fn builder() -> ShaderBuilder<Self, DataTy, EntriesTy>
    where
        Self: Sized,
    {
        ShaderBuilder::default()
    }
}
