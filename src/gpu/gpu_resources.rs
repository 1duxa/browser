use wgpu::{CommandEncoder, SurfaceTexture, TextureView};

use crate::{
    State,
    gpu::{sdf::SdfPipeline, solids::SolidsPipeline, text::TextPipeline},
};

pub struct GpuResources {
    pub common: Common,
    pub solids_pipe: SolidsPipeline,
    pub sdf_pipe: SdfPipeline,
    pub text_pipe: TextPipeline,
}
impl GpuResources {
    pub fn new(
        common: Common,
        solids_pipe: SolidsPipeline,
        sdf_pipe: SdfPipeline,
        text_pipe: TextPipeline,
    ) -> Self {
        Self {
            common,
            solids_pipe,
            sdf_pipe,
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
    pub fn get_encoder(&self) -> CommandEncoder {
        self.device.create_command_encoder(&Default::default())
    }

    pub fn get_texture(state: &mut State) -> Option<(SurfaceTexture, TextureView)> {
        let surface_texture = match state.gpu_resources.common.surface.get_current_texture() {
            wgpu::CurrentSurfaceTexture::Success(texture) => texture,
            wgpu::CurrentSurfaceTexture::Occluded | wgpu::CurrentSurfaceTexture::Timeout => {
                return None;
            }
            wgpu::CurrentSurfaceTexture::Suboptimal(_) | wgpu::CurrentSurfaceTexture::Outdated => {
                state.configure_surface();
                return None;
            }
            wgpu::CurrentSurfaceTexture::Validation => {
                unreachable!("No error scope registered, so validation errors will panic")
            }
            wgpu::CurrentSurfaceTexture::Lost => {
                state.gpu_resources.common.surface =
                    state.instance.create_surface(state.window.clone()).unwrap();
                state.configure_surface();
                return None;
            }
        };

        let view = surface_texture
            .texture
            .create_view(&wgpu::TextureViewDescriptor {
                format: Some(state.gpu_resources.common.surface_format.add_srgb_suffix()),
                ..Default::default()
            });

        Some((surface_texture, view))
    }
}
