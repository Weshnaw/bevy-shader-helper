use std::{fmt::Debug, marker::PhantomData};

use bevy_render::render_graph::RenderLabel;

#[derive(RenderLabel)]
pub(super) struct ShaderLabel<T> {
    _phantom: PhantomData<T>,
}

impl<T> ShaderLabel<T> {
    pub(super) fn new() -> Self {
        Self {
            _phantom: Default::default(),
        }
    }
}

impl<T> Debug for ShaderLabel<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ShaderLabel")
            .field("_phantom", &self._phantom)
            .finish()
    }
}

impl<T> Clone for ShaderLabel<T> {
    fn clone(&self) -> Self {
        Self {
            _phantom: self._phantom.clone(),
        }
    }
}

impl<T> std::hash::Hash for ShaderLabel<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self._phantom.hash(state);
    }
}

impl<T> PartialEq for ShaderLabel<T> {
    fn eq(&self, other: &Self) -> bool {
        self._phantom == other._phantom
    }
}

impl<T> Eq for ShaderLabel<T> {}

// impl<T: Send + Sync + 'static> RenderLabel for ShaderLabel<T> {
//     #[doc = r" Clones this `"]
//     #[doc = stringify!(RenderLabel)]
//     #[doc = r"`."]
//     fn dyn_clone(&self) -> label::Box<dyn RenderLabel> {
//         label::Box::new(self.clone())
//         // todo!()
//     }

//     #[doc = r" Casts this value to a form where it can be compared with other type-erased values."]
//     fn as_dyn_eq(&self) -> &dyn label::DynEq {
//         // self._phantom.e
//         todo!()
//     }

//     #[doc = r" Feeds this value into the given [`Hasher`]."]
//     fn dyn_hash(&self, state: &mut dyn ::core::hash::Hasher) {
//         todo!()
//     }
// }
