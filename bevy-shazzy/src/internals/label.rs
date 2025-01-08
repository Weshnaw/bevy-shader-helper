use std::marker::PhantomData;

use bevy_render::render_graph::RenderLabel;

#[derive(Debug, Hash, PartialEq, Eq, Clone, RenderLabel)]
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
