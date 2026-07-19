use std::sync::Arc;
use anyhow::Result;
use wgpu::util::DeviceExt;
use winit::window::Window;
use crate::vertex::Vertex;

pub struct State {
    device: wgpu::Device,
    queue: wgpu::Queue,
    surface: wgpu::Surface<'static>,
    config: wgpu::SurfaceConfiguration,
    pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    n_vertices: u32
}

impl State {
    pub async fn new<T: Vertex>(
        window: Arc<Window>,
        shader_src: &str,
        vertices: &Vec<T>
    ) -> Result<Self> {
        let instance = wgpu::Instance::default();
        let surface = instance.create_surface(window.clone())?;
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions::default())
            .await?;
        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor::default())
            .await?;
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(shader_src.into())
        });
        let format = surface.get_capabilities(&adapter).formats[0];
        let pipeline = device.create_render_pipeline(
            &wgpu::RenderPipelineDescriptor {
                vertex: wgpu::VertexState {
                    module: &shader,
                    entry_point: Some("vs_main"),
                    buffers: &[Some(T::LAYOUT)],
                    compilation_options: Default::default(),
                },
                fragment: Some(wgpu::FragmentState {
                    module: &shader,
                    entry_point: Some("fs_main"),
                    compilation_options: Default::default(),
                    targets: &[Some(format.into())],
                }),
                primitive: wgpu::PrimitiveState::default(),
                multisample: wgpu::MultisampleState::default(),
                label: None,
                layout: None,
                depth_stencil: None,
                multiview_mask: None,
                cache: None
            }
        );
        let vertex_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(vertices),
                usage: wgpu::BufferUsages::VERTEX
            }
        );
        let config = surface.get_default_config(&adapter, 1, 1).unwrap();
        let mut state = State {
            device,
            queue,
            surface,
            config,
            pipeline,
            vertex_buffer,
            n_vertices: vertices.len() as u32
        };
        let size = window.inner_size();
        state.resize(size.width, size.height);
        Ok(state)
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.config.width = width.max(1);
        self.config.height = height.max(1);
        self.surface.configure(&self.device, &self.config);
    }

    pub fn render(&self) {
        let frame = {
            use wgpu::CurrentSurfaceTexture::*;
            match self.surface.get_current_texture() {
                Success(x) | Suboptimal(x) => x,
                Timeout | Occluded | Validation => return,
                Outdated => {
                    self.surface.configure(&self.device, &self.config);
                    return
                },
                Lost => panic!("Lost")
            }
        };
        let view = frame.texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self.device.create_command_encoder(
            &wgpu::CommandEncoderDescriptor::default()
        );
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view,
                depth_slice: None,
                resolve_target: None,
                ops: wgpu::Operations::default()
            })],
            label: None,
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
            multiview_mask: None
        });
        pass.set_pipeline(&self.pipeline);
        pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        pass.draw(0..self.n_vertices, 0..1);
        drop(pass);
        self.queue.submit(Some(encoder.finish()));
        self.queue.present(frame);
    }
}

