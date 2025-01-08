use std::{fmt::Debug, marker::PhantomData};

use bevy_render::render_graph::RenderLabel;

#[derive(RenderLabel)]
pub(super) struct ShaderLabel<T>(PhantomData<T>);

impl<T> ShaderLabel<T> {
    pub(super) fn new() -> Self {
        Self(Default::default())
    }
}

impl<T> Debug for ShaderLabel<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ShaderLabel")
            .field("phantom", &self.0)
            .finish()
    }
}

impl<T> Clone for ShaderLabel<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T> std::hash::Hash for ShaderLabel<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl<T> PartialEq for ShaderLabel<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<T> Eq for ShaderLabel<T> {}
