use std::{cell::RefCell, sync::Mutex};

pub static NEW_STATE: Mutex<Option<State>> = Mutex::new(None);
thread_local! {
    pub static STATE: RefCell<Option<State>> = RefCell::new(None);
}

pub struct State {
    pub instance: wgpu::Instance,
    pub adapter: wgpu::Adapter,
    pub shader: wgpu::ShaderModule,
    pub pipeline_layout: wgpu::PipelineLayout,
    pub surface: wgpu::Surface,
    pub device: wgpu::Device,
    pub render_pipeline: wgpu::RenderPipeline,
    pub queue: wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration,
    pub time: f64,
}
