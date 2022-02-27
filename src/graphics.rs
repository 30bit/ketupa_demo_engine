use {
    crate::layers::{Color, Instance, Layers},
    bytemuck::cast_slice,
    glam::{vec2, Vec2},
    std::{iter::once, mem::size_of},
    wgpu::*,
    winit::{dpi::PhysicalSize, window::Window},
};

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct Params {
    pub screen_half_recip: Vec2,
    pub screen_zoom: f32,
    dummy: f32,
}

impl Params {
    pub fn new(screen_width: u32, screen_height: u32, screen_zoom: f32) -> Self {
        let half = vec2(screen_width as _, screen_height as _) * 0.5;
        Self {
            screen_half_recip: half.recip(),
            screen_zoom,
            dummy: 0.0,
        }
    }

    pub fn into_array(self) -> [f32; 4] {
        [
            self.screen_half_recip.x,
            self.screen_half_recip.y,
            self.screen_zoom,
            self.dummy,
        ]
    }
}

pub(crate) struct Graphics {
    pub(crate) layers: Layers,
    surface: Surface,
    device: Device,
    queue: Queue,
    config: SurfaceConfiguration,
    vertex_buffer: Buffer,
    index_buffer: Buffer,
    instance_buffer: Buffer,
    params_buffer: Buffer,
    params_bind_group: BindGroup,
    pipeline: RenderPipeline,
}

fn create_buffer<T>(device: &Device, len: usize, usage: BufferUsages) -> Buffer {
    device.create_buffer(&BufferDescriptor {
        label: None,
        size: (len * size_of::<T>()) as _,
        usage: usage | BufferUsages::COPY_DST,
        mapped_at_creation: false,
    })
}

impl Graphics {
    pub async fn new(window: &Window, layers: Layers) -> Self {
        let instance = wgpu::Instance::new(Backends::all());
        let surface = unsafe { instance.create_surface(window) };
        let adapter = instance
            .request_adapter(&RequestAdapterOptions {
                compatible_surface: Some(&surface),
                ..Default::default()
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(&Default::default(), None)
            .await
            .unwrap();

        let size = window.inner_size();
        let config = SurfaceConfiguration {
            usage: TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_preferred_format(&adapter).unwrap(),
            width: size.width,
            height: size.height,
            present_mode: PresentMode::Fifo,
        };
        surface.configure(&device, &config);

        let shader = device.create_shader_module(&include_wgsl!("shader.wgsl"));

        let vertex_buffer =
            create_buffer::<Vec2>(&device, layers.vertices.len(), BufferUsages::VERTEX);
        let index_buffer = create_buffer::<u16>(&device, layers.indices.len(), BufferUsages::INDEX);
        let instance_buffer =
            create_buffer::<Instance>(&device, layers.instances.len(), BufferUsages::VERTEX);
        let params_buffer = create_buffer::<Params>(&device, 1, BufferUsages::UNIFORM);

        let params_bind_group_layout =
            device.create_bind_group_layout(&BindGroupLayoutDescriptor {
                entries: &[BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::VERTEX,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
                label: None,
            });

        let params_bind_group = device.create_bind_group(&BindGroupDescriptor {
            layout: &params_bind_group_layout,
            entries: &[BindGroupEntry {
                binding: 0,
                resource: params_buffer.as_entire_binding(),
            }],
            label: None,
        });

        let pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            bind_group_layouts: &[&params_bind_group_layout],
            ..Default::default()
        });

        let pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
            label: None,
            layout: Some(&pipeline_layout),
            vertex: VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[
                    VertexBufferLayout {
                        array_stride: size_of::<Vec2>() as _,
                        step_mode: VertexStepMode::Vertex,
                        attributes: &vertex_attr_array![0 => Float32x2],
                    },
                    VertexBufferLayout {
                        array_stride: size_of::<Instance>() as _,
                        step_mode: VertexStepMode::Instance,
                        attributes: &vertex_attr_array![1 => Float32x4, 2 => Float32x3],
                    },
                ],
            },
            fragment: Some(FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[ColorTargetState {
                    format: config.format,
                    blend: Some(BlendState::ALPHA_BLENDING),
                    write_mask: ColorWrites::ALL,
                }],
            }),
            primitive: PrimitiveState {
                topology: PrimitiveTopology::TriangleList,
                front_face: FrontFace::Cw,
                ..Default::default()
            },
            depth_stencil: None,
            multisample: Default::default(),
            multiview: None,
        });

        Self {
            layers,
            surface,
            device,
            queue,
            config,
            pipeline,
            vertex_buffer,
            index_buffer,
            instance_buffer,
            params_buffer,
            params_bind_group,
        }
    }

    pub fn render(
        &mut self,
        size: PhysicalSize<u32>,
        zoom: f32,
        clear_color: Color,
    ) -> Result<(), ()> {
        if (size.width != self.config.width || size.height != self.config.height)
            && size.width != 0
            && size.height != 0
        {
            self.config.width = size.width;
            self.config.height = size.height;
            self.surface.configure(&self.device, &self.config);
        }
        let output = match self.surface.get_current_texture() {
            Ok(output) => output,
            Err(error) => {
                match error {
                    wgpu::SurfaceError::Lost => {
                        self.surface.configure(&self.device, &self.config);
                    }
                    wgpu::SurfaceError::OutOfMemory => {
                        return Err(());
                    }
                    _ => eprintln!("{}", error),
                };
                return Ok(());
            }
        };
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor::default());

        self.queue
            .write_buffer(&self.vertex_buffer, 0, cast_slice(&self.layers.vertices));
        self.queue
            .write_buffer(&self.index_buffer, 0, cast_slice(&self.layers.indices));
        self.queue
            .write_buffer(&self.instance_buffer, 0, cast_slice(&self.layers.instances));
        self.queue.write_buffer(
            &self.params_buffer,
            0,
            cast_slice(&Params::new(self.config.width, self.config.height, zoom).into_array()),
        );

        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: None,
            color_attachments: &[RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: Operations {
                    load: LoadOp::Clear(color_convert(clear_color)),
                    store: true,
                },
            }],
            depth_stencil_attachment: None,
        });
        pass.set_pipeline(&self.pipeline);
        pass.set_bind_group(0, &self.params_bind_group, &[]);
        pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        pass.set_vertex_buffer(1, self.instance_buffer.slice(..));
        pass.set_index_buffer(self.index_buffer.slice(..), IndexFormat::Uint16);
        for range in self.layers.ranges.iter() {
            let index_range = range.index_range32();
            let instance_range = range.instance_range32();
            pass.draw_indexed(index_range, 0, instance_range);
        }
        drop(pass);
        self.queue.submit(once(encoder.finish()));
        output.present();
        Ok(())
    }
}

fn color_convert(color: Color) -> wgpu::Color {
    wgpu::Color {
        r: color.r as f64 / 255.0,
        g: color.g as f64 / 255.0,
        b: color.b as f64 / 255.0,
        a: color.a as f64 / 255.0,
    }
}
