use super::texture;
use legion::world::World;
use crate::graphics::{shader, pipeline};
use glsl_to_spirv::ShaderType;
use wgpu::BufferDescriptor;
use crate::graphics::types::Vertex;


const VERTEX_BUFFER_INIT_SIZE: usize = std::mem::size_of::<Vertex>() * 3 * 128;

// Vertex Buffer is fixed sized right now, this is not good and should be changed!
// Split functions up and make it dynamic

pub struct Renderer {
    pub surface: wgpu::Surface,
    pub size: winit::dpi::PhysicalSize<u32>,
    pub adapter: wgpu::Adapter,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub sc_desc: wgpu::SwapChainDescriptor,
    pub swap_chain: wgpu::SwapChain,
    pub depth_texture: texture::Texture,
    pub window: winit::window::Window,

    //following should be moved into their own
    vertex_buffer: wgpu::Buffer,
    vertex_data: Vec<Vertex>,
    index_buffer: wgpu::Buffer,
    num_indices: u32,
    render_pipeline: wgpu::RenderPipeline,
    // uniforms: uniforms::Uniforms,
    // uniform_bind_group: wgpu::BindGroup,
    // uniform_buffer: wgpu::Buffer,
}

impl Renderer {
    pub async fn new(
        window: winit::window::Window,
        size: winit::dpi::PhysicalSize<u32>,
    ) -> Self {
        let surface = wgpu::Surface::create(&window);
        let adapter = wgpu::Adapter::request(
            &wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::Default,
                compatible_surface: Some(&surface),
            },
            wgpu::BackendBit::PRIMARY,
        )
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                extensions: wgpu::Extensions {
                    anisotropic_filtering: false,
                },
                limits: wgpu::Limits::default(),
            })
            .await;

        let sc_desc = wgpu::SwapChainDescriptor {
            usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
            format: wgpu::TextureFormat::Bgra8UnormSrgb,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
        };

        let swap_chain = device.create_swap_chain(&surface, &sc_desc);

        let depth_texture = texture::Texture::new_depth(&device, &sc_desc, "depth_texture");

        
        // following should be moved into their own
        let vs_module = shader::create_shader_module(
            include_str!("../../assets/shaders/default_vertex.glsl"),
            ShaderType::Vertex,
            &device,
        );

        let fs_module = shader::create_shader_module(
            include_str!("../../assets/shaders/default_fragment.glsl"),
            ShaderType::Fragment,
            &device,
        );

        let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            bind_group_layouts: &[],
        });

        let render_pipeline = pipeline::create_render_pipeline(
            &device,
            &render_pipeline_layout,
            wgpu::PrimitiveTopology::TriangleList,
            &vs_module,
            &fs_module,
            sc_desc.format.clone(),
            texture::DEPTH_FORMAT,
            &[Vertex::desc()],
            true,
            "main",
        );

        let vertex_buffer_desc = wgpu::BufferDescriptor {
            label: Some("vertex_buffer"),
            size: VERTEX_BUFFER_INIT_SIZE  as wgpu::BufferAddress,
            usage: wgpu::BufferUsage::VERTEX | wgpu::BufferUsage::COPY_DST
        };

        let vertex_buffer = device.create_buffer(&vertex_buffer_desc);

        let index_buffer_desc = wgpu::BufferDescriptor {
            label: Some("index_buffer"),
            size: 10,
            usage: wgpu::BufferUsage::INDEX | wgpu::BufferUsage::COPY_DST
        };

        let index_buffer = device.create_buffer(&index_buffer_desc);

        Self {
            surface,
            size,
            adapter,
            device,
            queue,
            sc_desc,
            swap_chain,
            depth_texture,
            window,
            vertex_buffer,
            vertex_data: Vec::new(),
            index_buffer,
            num_indices: 0,
            render_pipeline,
            // uniforms: (),
            // uniform_bind_group: (),
            // uniform_buffer: ()
        }
    }

    // Physics change within a fixed interval but render is as often as possible
    pub fn update_render_data(&mut self, world: &World) {

        self.vertex_data = VERTICES.into();

        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Update Encoder"),
        });

        let staging_vertex_buffer = self.device.create_buffer_with_data(
            bytemuck::cast_slice(&self.vertex_data),
            wgpu::BufferUsage::WRITE_ALL | wgpu::BufferUsage::COPY_SRC,
        );

        encoder.copy_buffer_to_buffer(
            &staging_vertex_buffer,
            0,
            &self.vertex_buffer,
            0,
            (std::mem::size_of::<Vertex>() * self.vertex_data.len()) as wgpu::BufferAddress,
        );

        self.queue.submit(&[encoder.finish()]);
    }

    pub fn render(&mut self) {
        let frame = self
            .swap_chain
            .get_next_texture()
            .expect("Timeout getting texture");

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                    attachment: &frame.view,
                    resolve_target: None,
                    load_op: wgpu::LoadOp::Clear,
                    store_op: wgpu::StoreOp::Store,
                    clear_color: wgpu::Color {
                        r: 0.2,
                        g: 0.0,
                        b: 0.5,
                        a: 1.0,
                    }
                }],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachmentDescriptor {
                    attachment: &self.depth_texture.view,
                    depth_load_op: wgpu::LoadOp::Clear,
                    depth_store_op: wgpu::StoreOp::Store,
                    clear_depth: 1.0,
                    stencil_load_op: wgpu::LoadOp::Clear,
                    stencil_store_op: wgpu::StoreOp::Store,
                    clear_stencil: 0,
                })
            });

            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_vertex_buffer(0, &self.vertex_buffer, 0,0 );
            render_pass.draw(0..self.vertex_data.len() as u32, 0..1);
        }

        self.queue.submit(&[encoder.finish()]);
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.size = new_size;
        self.sc_desc.width = new_size.width;
        self.sc_desc.height = new_size.height;
        self.swap_chain = self.device.create_swap_chain(&self.surface, &self.sc_desc);
        self.depth_texture = texture::Texture::new_depth(&self.device, &self.sc_desc, "depth_texture");
    }
}

const VERTICES: &[Vertex] = &[
    Vertex { position: [-10.0, -5.0, -10.0], color: [1.0, 0.0, 0.0, 1.0] }, // 9 R
    Vertex { position: [-10.0, -5.0, 10.0], color: [0.0, 1.0, 0.0, 1.0] }, // 10  G
    Vertex { position: [10.0, -5.0, -10.0], color: [0.0, 0.0, 1.0, 1.0] },
];