use std::sync::Arc;
use wgpu_text::glyph_brush::{Section, Text};
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop, OwnedDisplayHandle},
    window::{Window, WindowId},
};

use crate::{
    cli::{Cli, RunMode},
    component::Component,
    gpu::{
        gpu_resources::{self, GpuResources},
        sdf::SdfPipeline,
        solids::SolidsPipeline,
        text::TextPipeline,
    },
    uniform::ShaderSquare,
};

mod cli;
mod component;
mod gpu;
mod scripting;
mod tree;
mod uniform;

struct State<'a> {
    instance: wgpu::Instance,
    window: Arc<Window>,
    cursor_pos: winit::dpi::PhysicalPosition<f64>,
    size: winit::dpi::PhysicalSize<u32>,

    components: Vec<Component<'a>>,

    gpu_resources: GpuResources,

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

        let gpu_common = gpu_resources::Common::new(device, queue, surface, surface_format);

        let solids_pipe =
            SolidsPipeline::default_pipeline(&gpu_common.surface_format, &gpu_common.device);

        let sdf_pipe =
            SdfPipeline::default_pipeline(&gpu_common.surface_format, &gpu_common.device);
        let text_pipe = TextPipeline::default_pipeline(&gpu_common.device, surface_format, size);

        let gpu_resources = GpuResources::new(gpu_common, solids_pipe, sdf_pipe, text_pipe);
        let state = State {
            instance,
            window,
            size,
            gpu_resources,
            components: State::test_get_components(&cli),
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
            format: self.gpu_resources.common.surface_format,
            view_formats: vec![self.gpu_resources.common.surface_format.add_srgb_suffix()],
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
            width: self.size.width,
            height: self.size.height,
            desired_maximum_frame_latency: 2,
            present_mode: wgpu::PresentMode::AutoVsync,
        };
        self.gpu_resources
            .common
            .surface
            .configure(&self.gpu_resources.common.device, &surface_config);
    }

    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.size = new_size;
        self.configure_surface();
        self.gpu_resources.text_pipe.brush.resize_view(
            new_size.width as f32,
            new_size.height as f32,
            &self.gpu_resources.common.queue,
        );
    }

    fn render(&mut self, _: &Cli) {
        let (surface_texture, texture_view) = match gpu_resources::Common::get_texture(self) {
            Some(tx) => tx,
            _ => return,
        };
        let mut square_vec: Vec<&ShaderSquare> = Vec::new();
        let mut sections: Vec<&Section> = Vec::new();
        for c in &self.components {
            match c {
                Component::Square(sq) => square_vec.push(sq),
                Component::Text(txt) => sections.push(txt),
            }
        }

        self.gpu_resources.solids_pipe.update(
            &self.gpu_resources.common.device,
            &self.gpu_resources.common.queue,
            &square_vec,
        );

        self.gpu_resources
            .text_pipe
            .brush
            .queue(
                &self.gpu_resources.common.device,
                &self.gpu_resources.common.queue,
                sections,
            )
            .unwrap();

        let mut encoder = self.gpu_resources.common.get_encoder();
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

        renderpass.set_pipeline(&self.gpu_resources.solids_pipe.pipeline);
        renderpass.set_bind_group(0, &self.gpu_resources.solids_pipe.bind_group, &[]);

        let squares = square_vec.len() as u32;

        renderpass.draw(0..(squares * ShaderSquare::VERTECIES), 0..squares);

        self.gpu_resources.text_pipe.brush.draw(&mut renderpass);

        drop(renderpass);

        self.gpu_resources.common.queue.submit([encoder.finish()]);
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
