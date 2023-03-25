use std::borrow::Cow;

use anyhow::{anyhow, Result};
use log::{error, info};
use raw_window_handle::{HasRawDisplayHandle, HasRawWindowHandle};
use tokio::runtime::Runtime;

use crate::state::{State, NEW_STATE, STATE};

// Initialize the wgpu on the texture
pub fn init_wgpu<S: HasRawDisplayHandle + HasRawWindowHandle>(surface: S) -> Result<()> {
    let instance = wgpu::Instance::default();
    let surface = unsafe { instance.create_surface(&surface) }.unwrap();
    let tokio_runtime = Runtime::new()?;
    let result: Result<()> = tokio_runtime.block_on(async {
        info!("Running a tokio runtime task - getting an adapter");
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                force_fallback_adapter: false,
                // Request an adapter which can render to our surface
                compatible_surface: Some(&surface),
            })
            .await
            .ok_or(anyhow!("Failed to find an appropriate adapter"))?;
        info!("Adapter: {:#?}", adapter.get_info());

        // Check what it supports
        let capabilities = adapter.get_downlevel_capabilities();
        info!("Adapter Capabilities: {:#?}", capabilities);

        // Device and queue
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    features: wgpu::Features::empty(),
                    // Make sure we use the texture resolution limits from the adapter, so we can support images the size of the swapchain.
                    limits: wgpu::Limits::downlevel_webgl2_defaults()
                        .using_resolution(adapter.limits()),
                },
                None,
            )
            .await
            .map_err(|e| anyhow!("Failed to create device: {:?}", e))?;
        info!("Device: {:#?}", device);

        // Set a custom error handler
        device.on_uncaptured_error(Box::new(move |error| {
            error!("Uncaptured WGPU error: {:?}", error);

            // If validation error, handle it prettier
            if let wgpu::Error::Validation {
                source,
                description,
            } = error
            {
                error!("Validation error: {}", description);
                error!("Validation error source: {:#?}", source);
            }
        }));

        // Load the shaders from disk
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("../shaders/shader.wgsl"))),
        });
        info!("Shader: {:#?}", shader);

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });
        info!("Pipeline layout: {:#?}", pipeline_layout);

        let swapchain_capabilities = surface.get_capabilities(&adapter);
        info!("Swapchain capabilities: {:#?}", swapchain_capabilities);
        let swapchain_format = swapchain_capabilities.formats[0];
        info!("Swapchain format: {:#?}", swapchain_format);

        // Pipiline
        let size = [1, 1];
        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(swapchain_format.into())],
            }),
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
        });

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: swapchain_format,
            width: size[0],
            height: size[1],
            present_mode: wgpu::PresentMode::Mailbox,
            alpha_mode: swapchain_capabilities.alpha_modes[0],
            view_formats: vec![],
        };
        surface.configure(&device, &config);

        // Create and set the state
        let state = State {
            adapter,
            instance,
            pipeline_layout,
            shader,
            surface,
            device,
            render_pipeline,
            queue,
            config,
            time: 0.0,
        };
        let mut new_state_lock = NEW_STATE
            .lock()
            .map_err(|e| anyhow!("Failed to lock new state: {:?}", e))?;
        *new_state_lock = Some(state);
        info!(
            "Initialized WGPU state on thread {:?}",
            std::thread::current().id()
        );

        Ok(())
    });
    result?;

    Ok(())
}

pub fn move_state_to_ui_thread() -> Result<()> {
    let mut new_state_lock = NEW_STATE
        .lock()
        .map_err(|e| anyhow!("Failed to lock new state: {:?}", e))?;
    let new_state = new_state_lock.take();
    STATE.with(|s| {
        let mut state = s.borrow_mut();
        *state = new_state;

        info!(
            "Moved WGPU state to UI thread {:?}",
            std::thread::current().id()
        );
    });

    Ok(())
}

pub fn render_frame() -> Result<()> {
    STATE.with(|s| {
        info!(
            "Rendering frame on thread {:?}",
            std::thread::current().id()
        );

        info!("Getting state");
        let state = s.borrow();
        info!("Got state");
        let state = state.as_ref().ok_or(anyhow!("No state"))?;
        info!("Got state as ref");
        let instance = &state.instance;
        let adapter = &state.adapter;
        let pipeline_layout = &state.pipeline_layout;
        let shader = &state.shader;
        let surface = &state.surface;
        let device = &state.device;
        let render_pipeline = &state.render_pipeline;
        let queue = &state.queue;
        info!("Borrowed everything");

        // Draw one frame
        let frame = surface
            .get_current_texture()
            .map_err(|e| anyhow!("Failed to get current texture: {:?}", e))?;
        info!("Got frame");
        let view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        info!("Got view");
        let mut encoder =
            device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        info!("Created encoder");
        {
            let mut clear_color = wgpu::Color::default();
            clear_color.r = (f64::sin(state.time / 300.0) + 1.0) / 2.0;
            clear_color.g = (f64::cos(state.time / 300.0) + 1.0) / 2.0;
            clear_color.b = (f64::sin(state.time / 300.0 * 1.5) + 1.0) / 2.0;
            clear_color.a = 1.0;

            let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(clear_color),
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            });
            info!("Setting pipeline");
            rpass.set_pipeline(&render_pipeline);
            info!("Drawing");
            rpass.draw(0..3, 0..1);
        }

        info!("Submitting frame");
        queue.submit(Some(encoder.finish()));
        info!("Presenting frame");
        frame.present();
        // std::thread::sleep(std::time::Duration::from_millis(2000));

        info!("Done drawing one frame");

        Ok(())
    })
}

pub fn receive_resize(width: u32, height: u32) -> Result<()> {
    STATE.with(|s| {
        info!("Resizing to {}x{}", width, height);
        let mut state = s.borrow_mut();
        let state = state.as_mut().unwrap();
        let instance = &state.instance;
        let adapter = &state.adapter;
        let pipeline_layout = &state.pipeline_layout;
        let shader = &state.shader;
        let surface = &state.surface;
        let device = &state.device;
        let render_pipeline = &state.render_pipeline;
        let queue = &state.queue;
        let config = &mut state.config;

        // Reconfigure the surface with the new size
        config.width = width;
        config.height = height;
        surface.configure(&device, &config);
        // On macos the window needs to be redrawn manually after resizing
        // window.request_redraw();

        Ok(())
    })
}
