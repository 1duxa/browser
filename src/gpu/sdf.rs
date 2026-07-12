use wgpu::{BindGroup, Buffer, RenderPipeline};

use crate::{gpu::vec_buf::VecBuf, uniform::ShaderSdfRect};

pub struct SdfPipeline {
    pub pipeline: RenderPipeline,
    bind_group_layout: wgpu::BindGroupLayout,
    pub bind_group: BindGroup,
    pub buffer: VecBuf<ShaderSdfRect>,
}

impl SdfPipeline {
    const INITIAL_CAPACITY: u64 = 4;

    pub fn default_pipeline(surface_format: &wgpu::TextureFormat, device: &wgpu::Device) -> Self {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("sdf_shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("../../sdf.wgsl").into()),
        });

        let buffer = VecBuf::<ShaderSdfRect>::with_capacity(device, Self::INITIAL_CAPACITY);

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

        let bind_group = Self::make_bind_group(device, &bind_group_layout, buffer.buf());

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[Some(&bind_group_layout)],
            immediate_size: 0,
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("sdf_pipeline"),
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
                    blend: Some(wgpu::BlendState::PREMULTIPLIED_ALPHA_BLENDING),
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

        Self {
            pipeline,
            bind_group_layout,
            bind_group,
            buffer,
        }
    }

    fn make_bind_group(
        device: &wgpu::Device,
        layout: &wgpu::BindGroupLayout,
        buf: &Buffer,
    ) -> BindGroup {
        device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: buf.as_entire_binding(),
            }],
        })
    }

    pub fn update(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        rects: &Vec<&ShaderSdfRect>,
    ) {
        let grew = self.buffer.write(device, queue, rects, rects.len() as u64);
        if grew {
            self.bind_group =
                Self::make_bind_group(device, &self.bind_group_layout, self.buffer.buf());
        }
    }
}
