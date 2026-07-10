use std::sync::Arc;
use wgpu::{BindGroup, Buffer, RenderPipeline};
use wgpu_text::{
    BrushBuilder, TextBrush,
    glyph_brush::{Section, Text, ab_glyph::FontArc},
};
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop, OwnedDisplayHandle},
    window::{Window, WindowId},
};

use crate::{
    cli::{Cli, RunMode},
    component::Component,
    uniform::ShaderSquare,
};

mod cli;
mod component;
mod scripting;
mod uniform;

struct State<'a> {
    instance: wgpu::Instance,
    window: Arc<Window>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    size: winit::dpi::PhysicalSize<u32>,
    surface: wgpu::Surface<'static>,
    surface_format: wgpu::TextureFormat,
    pipeline: RenderPipeline,
    bind_group: BindGroup,
    buffer: Buffer,
    components: Vec<Component<'a>>,
    brush: TextBrush,
    cursor_pos: winit::dpi::PhysicalPosition<f64>,
    hovered: Option<usize>,
}

impl<'a> State<'a> {
    async fn new(display: OwnedDisplayHandle, window: Arc<Window>, cli: Arc<Cli>) -> State<'a> {
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::new_with_display_handle(
            Box::new(display),
        ));
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions::default())
            .await
            .unwrap();
        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor::default())
            .await
            .unwrap();

        let size = window.inner_size();

        let surface = instance.create_surface(window.clone()).unwrap();
        let cap = surface.get_capabilities(&adapter);
        let surface_format = cap.formats[0];

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("../square.wgsl").into()),
        });
        let uniform_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: 3 * <ShaderSquare as encase::ShaderType>::min_size().get(),
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: None,
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Storage { read_only: true },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding(),
            }],
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[Some(&bind_group_layout)],
            immediate_size: 0,
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format: surface_format.add_srgb_suffix(),
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
            }),
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            cache: None,
            multiview_mask: None,
        });

        let font = FontArc::try_from_vec(include_bytes!("../DejaVuSans.ttf").to_vec()).unwrap();
        let brush = BrushBuilder::using_font(font).build(
            &device,
            size.width.max(1),
            size.height.max(1),
            surface_format,
        );

        let state = State {
            instance,
            window,
            device,
            queue,
            size,
            surface,
            surface_format,
            pipeline,
            bind_group,
            components: State::test_get_components(&cli),
            buffer: uniform_buffer,
            brush,
            cursor_pos: (0.0, 0.0).into(),
            hovered: None,
        };

        state.configure_surface();

        state
    }

    fn test_get_components(cli: &Cli) -> Vec<Component<'static>> {
        match cli.mode {
            RunMode::SquareTest => vec![
                Component::Square(ShaderSquare {
                    scale: 1.0,
                    width: 0.5,
                    height: 0.5,
                    x: 0.5,
                    y: -0.5,
                    z: 0.0,
                    w: 1.0,
                    r: 0.0,
                    g: 1.0,
                    b: 0.0,
                    a: 1.0,
                    ..Default::default()
                }),
                Component::Square(ShaderSquare {
                    scale: 1.0,
                    width: 0.5,
                    height: 0.5,
                    x: -0.5,
                    y: 0.5,
                    z: 0.0,
                    w: 1.0,
                    r: 1.0,
                    g: 0.0,
                    b: 0.0,
                    a: 1.0,
                    ..Default::default()
                }),
                Component::Square(ShaderSquare {
                    scale: 1.0,
                    width: 0.5,
                    height: 0.5,
                    x: 0.5,
                    y: 0.5,
                    z: 0.0,
                    w: 1.0,
                    r: 0.0,
                    g: 0.0,
                    b: 1.0,
                    a: 1.0,
                    ..Default::default()
                }),
                Component::Text(
                    Section::default()
                        .add_text(Text::new("HELLO WORLD!!!!").with_scale(21.0))
                        .with_screen_position((4.0, 10.0)),
                ),
            ],
            RunMode::Default => Vec::new(),
        }
    }

    fn get_window(&self) -> &Window {
        &self.window
    }

    fn configure_surface(&self) {
        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: self.surface_format,
            view_formats: vec![self.surface_format.add_srgb_suffix()],
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
            width: self.size.width,
            height: self.size.height,
            desired_maximum_frame_latency: 2,
            present_mode: wgpu::PresentMode::AutoVsync,
        };
        self.surface.configure(&self.device, &surface_config);
    }

    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.size = new_size;
        self.configure_surface();
        self.brush
            .resize_view(new_size.width as f32, new_size.height as f32, &self.queue);
    }

    fn render(&mut self, _: &Cli) {
        let surface_texture = match self.surface.get_current_texture() {
            wgpu::CurrentSurfaceTexture::Success(texture) => texture,
            wgpu::CurrentSurfaceTexture::Occluded | wgpu::CurrentSurfaceTexture::Timeout => return,
            wgpu::CurrentSurfaceTexture::Suboptimal(_) | wgpu::CurrentSurfaceTexture::Outdated => {
                self.configure_surface();
                return;
            }
            wgpu::CurrentSurfaceTexture::Validation => {
                unreachable!("No error scope registered, so validation errors will panic")
            }
            wgpu::CurrentSurfaceTexture::Lost => {
                self.surface = self.instance.create_surface(self.window.clone()).unwrap();
                self.configure_surface();
                return;
            }
        };
        let texture_view = surface_texture
            .texture
            .create_view(&wgpu::TextureViewDescriptor {
                format: Some(self.surface_format.add_srgb_suffix()),
                ..Default::default()
            });

        let mut square_vec: Vec<&ShaderSquare> = Vec::new();
        let mut sections: Vec<&Section> = Vec::new();
        for c in &self.components {
            match c {
                Component::Square(sq) => square_vec.push(sq),
                Component::Text(txt) => sections.push(txt),
            }
        }

        let mut data = encase::StorageBuffer::new(Vec::new());
        data.write(&square_vec).unwrap();
        self.queue.write_buffer(&self.buffer, 0, data.as_ref());

        self.brush
            .queue(&self.device, &self.queue, sections)
            .unwrap();

        let mut encoder = self.device.create_command_encoder(&Default::default());
        let mut renderpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: None,
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &texture_view,
                depth_slice: None,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::WHITE),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
            multiview_mask: None,
        });

        renderpass.set_pipeline(&self.pipeline);
        renderpass.set_bind_group(0, &self.bind_group, &[]);

        let squares = square_vec.len() as u32;

        renderpass.draw(0..(squares * ShaderSquare::VERTECIES), 0..squares);

        self.brush.draw(&mut renderpass);

        drop(renderpass);

        self.queue.submit([encoder.finish()]);
        self.window.pre_present_notify();
        surface_texture.present();
    }

    fn cursor_to_ndc(&self) -> (f32, f32) {
        let x = (self.cursor_pos.x as f32 / self.size.width as f32) * 2.0 - 1.0;
        let y = 1.0 - (self.cursor_pos.y as f32 / self.size.height as f32) * 2.0; // flip y
        (x, y)
    }
}

#[derive(Default)]
struct App<'a> {
    state: Option<State<'a>>,
    cli: Arc<Cli>,
}

impl<'a> ApplicationHandler for App<'a> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = Arc::new(
            event_loop
                .create_window(Window::default_attributes())
                .unwrap(),
        );

        let state = pollster::block_on(State::new(
            event_loop.owned_display_handle(),
            window.clone(),
            self.cli.clone(),
        ));
        self.state = Some(state);

        window.request_redraw();
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        let state = self.state.as_mut().unwrap();
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                state.render(&self.cli);
                state.get_window().request_redraw();
            }
            WindowEvent::MouseInput {
                state: element_state,
                button,
                ..
            } if element_state == winit::event::ElementState::Pressed
                && button == winit::event::MouseButton::Left =>
            {
                let (nx, ny) = state.cursor_to_ndc();
                for c in state.components.iter().rev() {
                    if let Component::Square(sq) = c {
                        let hw = sq.width * sq.scale / 2.0;
                        let hh = sq.height * sq.scale / 2.0;
                        if (nx - sq.x).abs() <= hw && (ny - sq.y).abs() <= hh {
                            _ = scripting::test::build();
                            break;
                        }
                    }
                }
            }
            WindowEvent::Resized(size) => {
                state.resize(size);
            }
            WindowEvent::CursorMoved { position, .. } => {
                state.cursor_pos = position;
                let (nx, ny) = state.cursor_to_ndc();
                state.hovered = state
                    .components
                    .iter()
                    .enumerate()
                    .rev()
                    .find_map(|(i, c)| {
                        if let Component::Square(sq) = c {
                            let hw = sq.width * sq.scale / 2.0;
                            let hh = sq.height * sq.scale / 2.0;
                            if (nx - sq.x).abs() <= hw && (ny - sq.y).abs() <= hh {
                                return Some(i);
                            }
                        }
                        None
                    });
            }
            _ => (),
        }
    }
}

fn main() {
    env_logger::init();

    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = App::<'_> {
        cli: Arc::new(<Cli as clap::Parser>::parse()),
        ..Default::default()
    };

    event_loop.run_app(&mut app).unwrap();
}
