use winit::{
    event::*,
    event_loop::EventLoop,
    keyboard::{KeyCode, PhysicalKey},
    window::WindowBuilder,
};

use std::sync::Arc;
use winit::window::Window;

pub struct State<'a> {
    pub surface: wgpu::Surface<'a>,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration,
    pub size: winit::dpi::PhysicalSize<u32>,
    pub window: Arc<Window>, // wrapped window in an Arc so that I can create it inside State::new()
    pub surface_configured: bool,
    pub render_pipeline: wgpu::RenderPipeline,

    // ADDED BY ME !! :
    pub render_view: Option<wgpu::TextureView>,
    pub render_encoder: Option<wgpu::CommandEncoder>,
    pub render_pass: Option<wgpu::RenderPass<'a>>,
}

impl<'a> State<'a> {
    pub async fn new(event_loop: &EventLoop<()>) -> State<'a> {
        let window = WindowBuilder::new().build(&event_loop).unwrap();

        let window_arc = Arc::new(window);
        let size = window_arc.inner_size();

        // The instance is a handle to our GPU
        // BackendBit::PRIMARY => Vulkan + Metal + DX12 + Browser WebGPU
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::PRIMARY,
            ..Default::default()
        });

        let surface = instance.create_surface(window_arc.clone()).unwrap();

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    required_features: wgpu::Features::empty(),
                    required_limits: wgpu::Limits::default(),
                },
                None, // Trace path
            )
            .await
            .unwrap();

        let surface_caps = surface.get_capabilities(&adapter);
        // Shader code in this tutorial assumes an Srgb surface texture. Using a different
        // one will result all the colors comming out darker. If you want to support non
        // Srgb surfaces, you'll need to account for that when drawing to the frame.

        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            desired_maximum_frame_latency: 2,
            view_formats: vec![],
        };

        let shader = device.create_shader_module(wgpu::include_wgsl!("shader.wgsl"));

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[],
                push_constant_ranges: &[],
            });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState {
                        color: wgpu::BlendComponent::REPLACE,
                        alpha: wgpu::BlendComponent::REPLACE,
                    }),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),

            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: None,
                // cull_mode: Some(wgpu::Face::Back),
                // Setting this to anything other than Fill requires Features::POLYGON_MODE_LINE
                // or Features::POLYGON_MODE_POINT
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false, // Requires Features::DEPTH_CLIP_CONTROL
                conservative: false,    // Requires Features::CONSERVATIVE_RASTERIZATION
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            // If the pipeline will be used with a multiview render pass, this
            // indicates how many array layers the attachments will have.
            multiview: None,
        });

        Self {
            surface,
            device,
            queue,
            config,
            size,
            window: window_arc,
            surface_configured: false,
            render_pipeline,

            render_view: None,
            render_encoder: None,
            render_pass: None,
        }
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.size.width = new_size.width.max(1);
        self.size.height = new_size.height.max(1);
        self.config.width = self.size.width;
        self.config.height = self.size.height;
        self.surface.configure(&self.device, &self.config);
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        self.render_view = Some(
            output
                .texture
                .create_view(&wgpu::TextureViewDescriptor::default()),
        );

        self.render_encoder = Some(self.device.create_command_encoder(
            &wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            },
        ));

        // the result of as_mut.unwrap only lasts for the duration of the function call apparently
        // but render_pass lives for 'a ?

        self.render_pass = Some(self.render_encoder.as_mut().unwrap().begin_render_pass(
            &wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &self.render_view.as_ref().unwrap(),
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            },
        ));

        self.render_pass
            .as_mut()
            .unwrap()
            .set_pipeline(&self.render_pipeline);

        self.render_pass.as_mut().unwrap().draw(0..3, 0..1);

        // ideally I would put this in a separate `finish_render()` function
        // then between the `render` and `finish_render` functions, the user could call the draw functions

        self.queue.submit(std::iter::once(
            self.render_encoder.as_mut().unwrap().finish(),
        ));
        output.present();
        Ok(())
    }

    pub fn request_redraw(&mut self, control_flow: &winit::event_loop::EventLoopWindowTarget<()>) {
        // This tells winit that we want another frame after this one
        self.window.request_redraw();

        if !self.surface_configured {
            return;
        }

        // state.update();
        match self.render() {
            Ok(_) => {}

            // Reconfigure the surface if it's lost or outdated
            Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => self.resize(self.size),

            // The system is out of memory, we should probably quit
            Err(wgpu::SurfaceError::OutOfMemory) => {
                log::error!("OutOfMemory");
                control_flow.exit();
            }

            // This happens when the a frame takes too long to present
            Err(wgpu::SurfaceError::Timeout) => log::warn!("Surface timeout"),
        }
    }

    pub fn handle_events(
        &mut self,
        event: &WindowEvent,
        control_flow: &winit::event_loop::EventLoopWindowTarget<()>,
    ) {
        match event {
            WindowEvent::CloseRequested => control_flow.exit(),

            WindowEvent::Resized(physical_size) => {
                log::info!("Resized: {physical_size:?}");
                self.surface_configured = true;
                self.resize(*physical_size);
            }

            WindowEvent::RedrawRequested => self.request_redraw(control_flow),

            _ => {}
        }
    }
}
