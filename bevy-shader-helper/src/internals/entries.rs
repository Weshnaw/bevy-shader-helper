use bevy::render::render_resource::{CachedPipelineState, ComputePass, PipelineCache};

use crate::internals::{binding::GenericBindGroup, pipeline::Pipeline};

pub use bevy_shader_macros::ShaderEntry;
pub trait ShaderEntry {
    fn as_key(&self) -> usize;
}

#[derive(Clone, Debug)]
pub struct Entry<EntryTy> {
    pub entry: EntryTy,
    pub workgroup: (u32, u32, u32),
}
impl<T> From<(T, u32, u32, u32)> for Entry<T> {
    fn from(value: (T, u32, u32, u32)) -> Self {
        Self {
            entry: value.0,
            workgroup: (value.1, value.2, value.3),
        }
    }
}
impl<T, V: Into<(u32, u32, u32)>> From<(T, V)> for Entry<T> {
    fn from(value: (T, V)) -> Self {
        Self {
            entry: value.0,
            workgroup: value.1.into(),
        }
    }
}

// TODO impl From (T, 1, 2, 3) / (T, (1, 2, 3))

impl<EntryTy: ShaderEntry> Entry<EntryTy> {
    fn get_state<'a, PipelineTy: Pipeline>(
        &'a self,
        pipeline_cache: &'a PipelineCache,
        pipeline: &'a PipelineTy,
    ) -> &'a CachedPipelineState {
        pipeline_cache.get_compute_pipeline_state(pipeline.get_id(&self.entry))
    }

    fn dispatch<PipelineTy: Pipeline>(
        &self,
        pipeline_cache: &PipelineCache,
        pipeline: &PipelineTy,
        pass: &mut ComputePass,
        bind_group: &GenericBindGroup<PipelineTy>,
    ) {
        if let Some(pipeline) = pipeline_cache.get_compute_pipeline(pipeline.get_id(&self.entry)) {
            pass.set_bind_group(0, &bind_group.0, &[]);
            pass.set_pipeline(pipeline);
            pass.dispatch_workgroups(self.workgroup.0, self.workgroup.1, self.workgroup.2);
        }
    }
}

#[derive(Clone)]
pub(crate) struct Dispatch<EntryTy> {
    pub on_startup: Vec<Entry<EntryTy>>,
    pub on_update: Vec<Entry<EntryTy>>,
    // TODO: on_request: Vec<(receiver, ShaderDispatch)>
}

impl<T, E1: Into<Vec<Entry<T>>>, E2: Into<Vec<Entry<T>>>> From<(E1, E2)> for Dispatch<T> {
    fn from(value: (E1, E2)) -> Self {
        Self {
            on_startup: value.0.into(),
            on_update: value.1.into(),
        }
    }
}

// TODO: consider refactoring so that you pass in a enum to specify if on_update or on_startup
impl<EntryTy: ShaderEntry> Dispatch<EntryTy> {
    pub(super) fn on_startup_success<PipelineTy: Pipeline>(
        &self,
        pipeline_cache: &PipelineCache,
        pipeline: &PipelineTy,
    ) -> bool {
        self.on_startup
            .iter()
            .map(|entry| entry.get_state(pipeline_cache, pipeline))
            .all(|state| match state {
                CachedPipelineState::Ok(_) => true,
                CachedPipelineState::Err(e) => {
                    panic!("Failed to load shader: {e}")
                }
                _ => false,
            })
    }

    pub(super) fn on_update_success<PipelineTy: Pipeline>(
        &self,
        pipeline_cache: &PipelineCache,
        pipeline: &PipelineTy,
    ) -> bool {
        self.on_startup
            .iter()
            .map(|entry| pipeline_cache.get_compute_pipeline_state(pipeline.get_id(&entry.entry)))
            .all(|state| matches!(state, CachedPipelineState::Ok(_)))
    }

    pub(super) fn on_startup_dispatch<PipelineTy: Pipeline>(
        &self,
        pipeline_cache: &PipelineCache,
        pipeline: &PipelineTy,
        pass: &mut ComputePass,
        bind_group: &GenericBindGroup<PipelineTy>,
    ) {
        for entry in self.on_startup.iter() {
            entry.dispatch(pipeline_cache, pipeline, pass, bind_group);
        }
    }

    pub(super) fn on_update_dispatch<PipelineTy: Pipeline>(
        &self,
        pipeline_cache: &PipelineCache,
        pipeline: &PipelineTy,
        pass: &mut ComputePass,
        bind_group: &GenericBindGroup<PipelineTy>,
    ) {
        for entry in self.on_update.iter() {
            entry.dispatch(pipeline_cache, pipeline, pass, bind_group);
        }
    }
}
