use wgpu::{BindGroup, Buffer, RenderPipeline};
use wgpu_text::{BrushBuilder, TextBrush, glyph_brush::ab_glyph::FontArc};
use winit::dpi::PhysicalSize;

use crate::uniform::ShaderSquare;

pub struct GpuResources {
    pub common: Common,
    pub solids_pipe: SolidsPipeline,
    pub text_pipe: TextPipeline,
}
impl GpuResources {
    pub fn new(common: Common, solids_pipe: SolidsPipeline, text_pipe: TextPipeline) -> Self {
        Self {
            common,
            solids_pipe,
            text_pipe,
        }
    }
}
pub struct Common {
    pub(crate) device: wgpu::Device,
    pub(crate) queue: wgpu::Queue,
    pub(crate) surface: wgpu::Surface<'static>,
    pub(crate) surface_format: wgpu::TextureFormat,
}
impl Common {
    pub fn new(
        device: wgpu::Device,
        queue: wgpu::Queue,
        surface: wgpu::Surface<'static>,
        surface_format: wgpu::TextureFormat,
    ) -> Self {
        Self {
            device,
            queue,
            surface,
            surface_format,
        }
    }
}

pub struct SolidsPipeline {
    pub pipeline: RenderPipeline,
    pub bind_group: BindGroup,
    pub buffer: Buffer,
}
impl SolidsPipeline {
    pub fn new(pipeline: RenderPipeline, bind_group: BindGroup, buffer: Buffer) -> Self {
        Self {
            pipeline,
            bind_group,
            buffer,
        }
    }
    pub fn default_pipeline(surface_format: &wgpu::TextureFormat, device: &wgpu::Device) -> Self {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("../../square.wgsl").into()),
        });
        let storage_buffer = device.create_buffer(&wgpu::BufferDescriptor {
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
                resource: storage_buffer.as_entire_binding(),
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

        Self::new(pipeline, bind_group, storage_buffer)
    }
}
pub struct TextPipeline {
    pub brush: TextBrush,
}
impl TextPipeline {
    pub fn default_pipeline(
        device: &wgpu::Device,
        surface_format: wgpu::TextureFormat,
        size: PhysicalSize<u32>,
    ) -> Self {
        let font = FontArc::try_from_vec(include_bytes!("../../DejaVuSans.ttf").to_vec()).unwrap();
        let brush = BrushBuilder::using_font(font).build(
            device,
            size.width.max(1),
            size.height.max(1),
            surface_format,
        );
        Self { brush }
    }
}
