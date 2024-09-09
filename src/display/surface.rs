use std::borrow::Cow;
use std::iter;
use std::mem;
use std::sync::Arc;
use std::time::Instant;

use wgpu::util::DeviceExt;

use winit::window::Window;

use super::window::WindowConfiguration;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct Vertex {
    vertex_coordinates: [f32; 3],
    texture_coordinates: [f32; 2],
}

const VERTICES: &[Vertex] = &[
    Vertex {
        vertex_coordinates: [-1.0, 1.0, 0.0],
        texture_coordinates: [0.0, 0.0],
    },
    Vertex {
        vertex_coordinates: [-1.0, -1.0, 0.0],
        texture_coordinates: [0.0, 1.0],
    },
    Vertex {
        vertex_coordinates: [1.0, -1.0, 0.0],
        texture_coordinates: [1.0, 1.0],
    },
    Vertex {
        vertex_coordinates: [1.0, 1.0, 0.0],
        texture_coordinates: [1.0, 0.0],
    },
];

const INDICES: &[u16] = &[0, 1, 2, 0, 2, 3];

const BACKGROUND: wgpu::Color = wgpu::Color {
    r: 0.0,
    g: 0.0,
    b: 0.0,
    a: 0.0,
};

const WGSL: &str = include_str!("shader.wgsl");

pub struct SurfaceState<'a> {
    buffer: Vec<u8>,
    instant: Instant,
    config: WindowConfiguration,
    window: Arc<Window>,
    surface: wgpu::Surface<'a>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    surface_configuration: wgpu::SurfaceConfiguration,
    render_pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    index_number: u32,
    bind_group: wgpu::BindGroup,
    texture: wgpu::Texture,
}

impl<'a> SurfaceState<'a> {
    pub async fn new(config: WindowConfiguration, window: Arc<Window>) -> SurfaceState<'a> {
        let length = (config.width * config.height * 4) as usize;

        let instant = Instant::now();

        let buffer = vec![0u8; length];

        let size = window.inner_size();

        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::PRIMARY,
            ..Default::default()
        });

        let surface = instance
            .create_surface(window.clone())
            .expect("WGPU instance should create surface with window");

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .expect("WGPU instance should retrieve adapter");

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: Some("Device Descriptor"),
                    required_features: wgpu::Features::empty(),
                    required_limits: wgpu::Limits::default(),
                    memory_hints: Default::default(),
                },
                None,
            )
            .await
            .expect("WGPU adapter should retrieve device and queue");

        let surface_capabilities = surface.get_capabilities(&adapter);

        let texture_format = surface_capabilities
            .formats
            .iter()
            .copied()
            .find(|format| format.is_srgb())
            .unwrap_or(surface_capabilities.formats[0]);

        let surface_configuration = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: texture_format,
            width: size.width,
            height: size.height,
            present_mode: surface_capabilities.present_modes[0],
            alpha_mode: surface_capabilities.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        let extend_3d = wgpu::Extent3d {
            width: config.width as u32,
            height: config.height as u32,
            depth_or_array_layers: 1,
        };

        let texture_format = wgpu::TextureFormat::Rgba8UnormSrgb;

        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Texture"),
            size: extend_3d,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: texture_format,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });

        queue.write_texture(
            wgpu::ImageCopyTexture {
                aspect: wgpu::TextureAspect::All,
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
            },
            &buffer,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(4 * config.width as u32),
                rows_per_image: Some(config.height as u32),
            },
            extend_3d,
        );

        let texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        let mut filter_mode = wgpu::FilterMode::Nearest;

        if config.filter {
            filter_mode = wgpu::FilterMode::Linear;
        }

        let sampler_descriptor = &wgpu::SamplerDescriptor {
            mag_filter: filter_mode,
            min_filter: filter_mode,
            mipmap_filter: filter_mode,
            ..Default::default()
        };

        // let sampler = device.create_sampler(&wgpu::SamplerDescriptor::default());

        let sampler = device.create_sampler(&sampler_descriptor);

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        multisampled: false,
                        view_dimension: wgpu::TextureViewDimension::D2,
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
            label: Some("Bind Group Layout"),
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&texture_view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&sampler),
                },
            ],
            label: Some("Bind Group"),
        });

        let shader_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader Module"),
            source: wgpu::ShaderSource::Wgsl(Cow::from(WGSL)),
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Pipeline Layout"),
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&pipeline_layout),
            cache: Default::default(),
            vertex: wgpu::VertexState {
                module: &shader_module,
                entry_point: "vs_main",
                buffers: &[wgpu::VertexBufferLayout {
                    array_stride: mem::size_of::<Vertex>() as wgpu::BufferAddress,
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &[
                        wgpu::VertexAttribute {
                            offset: 0,
                            shader_location: 0,
                            format: wgpu::VertexFormat::Float32x3,
                        },
                        wgpu::VertexAttribute {
                            offset: mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                            shader_location: 1,
                            format: wgpu::VertexFormat::Float32x2,
                        },
                    ],
                }],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader_module,
                entry_point: "fs_main",
                compilation_options: Default::default(),
                targets: &[Some(wgpu::ColorTargetState {
                    format: surface_configuration.format,
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
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        });

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(VERTICES),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(INDICES),
            usage: wgpu::BufferUsages::INDEX,
        });

        let index_number = INDICES.len() as u32;

        surface.configure(&device, &surface_configuration);

        println!("{}", adapter.get_info().name);

        Self {
            buffer,
            instant,
            config,
            window,
            surface,
            device,
            queue,
            surface_configuration,
            render_pipeline,
            vertex_buffer,
            index_buffer,
            index_number,
            bind_group,
            texture,
        }
    }

    fn write(&mut self, width: u32, height: u32) {
        self.queue.write_texture(
            wgpu::ImageCopyTexture {
                aspect: wgpu::TextureAspect::All,
                texture: &self.texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
            },
            &self.buffer,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(4 * width),
                rows_per_image: Some(height),
            },
            wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
        );
    }

    pub fn buffer(&mut self) -> &mut [u8] {
        &mut self.buffer
    }

    pub fn instant(&self) -> &Instant {
        &self.instant
    }

    pub fn window(&self) -> &Window {
        &self.window
    }

    pub fn resize(&mut self, size: winit::dpi::PhysicalSize<u32>) {
        if size.width > 0 && size.height > 0 {
            self.surface_configuration.width = size.width;
            self.surface_configuration.height = size.height;
            self.surface
                .configure(&self.device, &self.surface_configuration);
        }
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        self.write(self.config.width as u32, self.config.height as u32);

        let surface_texture = self.surface.get_current_texture()?;

        let texture_view = surface_texture
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut command_encoder =
            self.device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("Command Encoder"),
                });

        {
            let mut render_pass = command_encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &texture_view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(BACKGROUND),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });

            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_bind_group(0, &self.bind_group, &[]);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            render_pass.draw_indexed(0..self.index_number, 0, 0..1);
        }

        self.queue.submit(iter::once(command_encoder.finish()));

        surface_texture.present();

        Ok(())
    }
}
