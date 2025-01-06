use std::marker::PhantomData;

use bevy_asset::RenderAssetUsages;
use bevy_image::Image;
use bevy_render::render_resource::{Extent3d, TextureDimension, TextureFormat};

use crate::internals::entries::{Dispatch, Entry};
use crate::texture_details::{ToTextureDimension, ToTextureFormat};

#[derive(Clone, Default)]
pub enum ImageData {
    #[default]
    Zeros,
    Fill([u8; 4]),
    Data(Vec<u8>),
}

impl ImageData {
    fn image(self, size: Extent3d, format: TextureFormat, dimension: TextureDimension) -> Image {
        let asset_usage = RenderAssetUsages::RENDER_WORLD;
        match self {
            ImageData::Fill(data) => Image::new_fill(size, dimension, &data, format, asset_usage),
            ImageData::Data(vec) => Image::new(size, dimension, vec, format, asset_usage),
            ImageData::Zeros => {
                let total = size.height * size.width * size.depth_or_array_layers;
                let total = total * format.block_copy_size(None).unwrap_or(0);
                // debug!("Creating image of {total} size");
                Image::new(
                    size,
                    dimension,
                    vec![0; total as usize],
                    format,
                    asset_usage,
                )
            }
        }
    }
}

pub struct ImageBuilder<F, D> {
    pub size: Extent3d,
    pub data: ImageData,
    _phantom_dimension: PhantomData<F>,
    _phantom_format: PhantomData<D>,
}

impl<F, D> Clone for ImageBuilder<F, D> {
    fn clone(&self) -> Self {
        Self {
            size: self.size,
            data: self.data.clone(),
            _phantom_dimension: self._phantom_dimension,
            _phantom_format: self._phantom_format,
        }
    }
}

impl<F: ToTextureFormat, D: ToTextureDimension> From<ImageBuilder<F, D>> for Image {
    fn from(val: ImageBuilder<F, D>) -> Self {
        let dimension = D::texture_dimension();
        let format = F::texture_format();
        val.data.image(val.size, format, dimension)
    }
}

impl<F, D> From<Extent3d> for ImageBuilder<F, D> {
    fn from(value: Extent3d) -> Self {
        Self {
            size: value,
            data: Default::default(),
            _phantom_dimension: PhantomData,
            _phantom_format: PhantomData,
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
