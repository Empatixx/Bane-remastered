use std::error::Error;
use std::path::Path;
use std::sync::Arc;

use wgpu::util::DeviceExt;
use winit::dpi::PhysicalSize;
use winit::window::Window;

use super::atlas;
use super::light::Globals;
use super::post::PostUniform;
use super::texture::Pixels;
use super::{Draw, Frame, Quad, TextureId};

const MAX_QUADS: usize = 10_000;
const OCCLUDED_WAIT: std::time::Duration = std::time::Duration::from_millis(16);

#[derive(Debug)]
pub(crate) struct Gpu {
    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    pipeline: wgpu::RenderPipeline,
    globals_buffer: wgpu::Buffer,
    screen_bind_group: wgpu::BindGroup,
    texture_layout: wgpu::BindGroupLayout,
    sampler: wgpu::Sampler,
    textures: Vec<wgpu::BindGroup>,
    entries: Vec<TextureEntry>,
    quad_scratch: Vec<Quad>,
    skipped_frames: u32,
    quad_buffer: wgpu::Buffer,
    post: Post,
    targets: Option<Targets>,
}

#[derive(Debug, Clone, Copy)]
struct TextureEntry {
    group: usize,
    region: [f32; 4],
}

struct Post {
    params_buffer: wgpu::Buffer,
    params_group: wgpu::BindGroup,
    sampler: wgpu::Sampler,
    blit: wgpu::RenderPipeline,
    vignette: wgpu::RenderPipeline,
    blur: wgpu::RenderPipeline,
}

struct Targets {
    views: [wgpu::TextureView; 2],
    groups: [wgpu::BindGroup; 2],
    size: (u32, u32),
}

impl std::fmt::Debug for Post {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Post")
    }
}

impl std::fmt::Debug for Targets {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Targets").field("size", &self.size).finish()
    }
}

impl Gpu {
    pub(crate) fn new(window: Arc<Window>) -> Result<Self, Box<dyn Error>> {
        let size = window.inner_size();
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::new_without_display_handle());
        let surface = instance.create_surface(window)?;
        let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
            compatible_surface: Some(&surface),
            ..Default::default()
        }))?;
        let (device, queue) =
            pollster::block_on(adapter.request_device(&wgpu::DeviceDescriptor::default()))?;
        let config = surface
            .get_default_config(&adapter, size.width.max(1), size.height.max(1))
            .ok_or("the surface is not supported by this adapter")?;
        surface.configure(&device, &config);

        let globals_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("globals"),
            contents: bytemuck::bytes_of(&Globals::new(glam::Vec2::ZERO, &[])),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });
        let screen_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("screen"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });
        let screen_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("screen"),
            layout: &screen_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: globals_buffer.as_entire_binding(),
            }],
        });
        let texture_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("texture"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
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
        });
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("texture"),
            mag_filter: wgpu::FilterMode::Nearest,
            min_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });
        let quad_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("quads"),
            size: (MAX_QUADS * size_of::<Quad>()) as u64,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let shader = device.create_shader_module(wgpu::include_wgsl!("quad.wgsl"));
        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("quads"),
            bind_group_layouts: &[Some(&screen_layout), Some(&texture_layout)],
            immediate_size: 0,
        });
        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("quads"),
            layout: Some(&layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                compilation_options: wgpu::PipelineCompilationOptions::default(),
                buffers: &[Some(wgpu::VertexBufferLayout {
                    array_stride: size_of::<Quad>() as u64,
                    step_mode: wgpu::VertexStepMode::Instance,
                    attributes: &wgpu::vertex_attr_array![
                        0 => Float32x2,
                        1 => Float32x2,
                        2 => Float32x4,
                        3 => Float32x4,
                        4 => Float32x2,
                        5 => Float32x4,
                        6 => Float32x4,
                        7 => Float32x4,
                    ],
                })],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                compilation_options: wgpu::PipelineCompilationOptions::default(),
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview_mask: None,
            cache: None,
        });

        let post = Post::new(&device, config.format, &texture_layout);
        let mut gpu = Self {
            surface,
            device,
            queue,
            config,
            pipeline,
            globals_buffer,
            screen_bind_group,
            texture_layout,
            sampler,
            textures: Vec::new(),
            entries: Vec::new(),
            quad_scratch: Vec::new(),
            skipped_frames: 0,
            quad_buffer,
            post,
            targets: None,
        };
        gpu.upload(&Pixels::white());
        Ok(gpu)
    }

    pub(crate) fn load_texture(&mut self, path: &Path) -> Result<TextureId, Box<dyn Error>> {
        let pixels = Pixels::load(path)?;
        Ok(self.upload(&pixels))
    }

    pub(crate) fn upload_atlas(
        &mut self,
        images: Vec<(&'static str, Pixels)>,
    ) -> Result<Vec<(&'static str, TextureId)>, Box<dyn Error>> {
        let limit = self.device.limits().max_texture_dimension_2d;
        let sizes: Vec<(u32, u32)> = images
            .iter()
            .map(|(_, pixels)| (pixels.width, pixels.height))
            .collect();
        let packed = atlas::pack(&sizes, limit).ok_or("the textures do not fit into one atlas")?;
        let sources: Vec<(u32, u32, &[u8])> = images
            .iter()
            .map(|(_, pixels)| (pixels.width, pixels.height, pixels.rgba.as_slice()))
            .collect();
        let rgba = atlas::assemble(&packed, &sources);
        tracing::info!(
            textures = images.len(),
            size = packed.size,
            "packed the texture atlas"
        );
        let group = self.create_group(&Pixels {
            width: packed.size,
            height: packed.size,
            rgba,
        });
        let side = packed.size as f32;
        Ok(images
            .iter()
            .zip(&packed.positions)
            .map(|((name, pixels), (x, y))| {
                let region = [
                    *x as f32 / side,
                    *y as f32 / side,
                    pixels.width as f32 / side,
                    pixels.height as f32 / side,
                ];
                (*name, self.register(group, region))
            })
            .collect())
    }

    pub(crate) fn upload_alpha_mask(&mut self, width: u32, height: u32, alpha: &[u8]) -> TextureId {
        let rgba = alpha.iter().flat_map(|a| [255, 255, 255, *a]).collect();
        self.upload(&Pixels {
            width,
            height,
            rgba,
        })
    }

    fn upload(&mut self, pixels: &Pixels) -> TextureId {
        let group = self.create_group(pixels);
        self.register(group, [0.0, 0.0, 1.0, 1.0])
    }

    fn register(&mut self, group: usize, region: [f32; 4]) -> TextureId {
        self.entries.push(TextureEntry { group, region });
        TextureId::new(self.entries.len() - 1)
    }

    fn create_group(&mut self, pixels: &Pixels) -> usize {
        let size = wgpu::Extent3d {
            width: pixels.width,
            height: pixels.height,
            depth_or_array_layers: 1,
        };
        let texture = self.device.create_texture(&wgpu::TextureDescriptor {
            label: Some("texture"),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });
        self.queue.write_texture(
            texture.as_image_copy(),
            &pixels.rgba,
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(4 * pixels.width),
                rows_per_image: Some(pixels.height),
            },
            size,
        );
        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("texture"),
            layout: &self.texture_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&self.sampler),
                },
            ],
        });
        self.textures.push(bind_group);
        self.textures.len() - 1
    }

    pub(crate) fn set_vsync(&mut self, vsync: bool) {
        self.config.present_mode = if vsync {
            wgpu::PresentMode::Fifo
        } else {
            wgpu::PresentMode::AutoNoVsync
        };
        self.surface.configure(&self.device, &self.config);
        tracing::info!(present_mode = ?self.config.present_mode, "present mode set");
    }

    pub(crate) fn resize(&mut self, size: PhysicalSize<u32>) {
        if size.width == 0 || size.height == 0 {
            return;
        }
        self.config.width = size.width;
        self.config.height = size.height;
        self.surface.configure(&self.device, &self.config);
    }

    fn ensure_targets(&mut self) {
        let size = (self.config.width, self.config.height);
        if self
            .targets
            .as_ref()
            .is_some_and(|targets| targets.size == size)
        {
            return;
        }
        let make = |label: &str| {
            let texture = self.device.create_texture(&wgpu::TextureDescriptor {
                label: Some(label),
                size: wgpu::Extent3d {
                    width: size.0,
                    height: size.1,
                    depth_or_array_layers: 1,
                },
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: self.config.format,
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT
                    | wgpu::TextureUsages::TEXTURE_BINDING,
                view_formats: &[],
            });
            let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
            let group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some(label),
                layout: &self.texture_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(&view),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::Sampler(&self.post.sampler),
                    },
                ],
            });
            (view, group)
        };
        let (view_a, group_a) = make("target a");
        let (view_b, group_b) = make("target b");
        self.targets = Some(Targets {
            views: [view_a, view_b],
            groups: [group_a, group_b],
            size,
        });
    }

    fn quads_pass(
        &self,
        encoder: &mut wgpu::CommandEncoder,
        view: &wgpu::TextureView,
        load: wgpu::LoadOp<wgpu::Color>,
        draws: &[Draw],
        first_instance: u32,
    ) {
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view,
                depth_slice: None,
                resolve_target: None,
                ops: wgpu::Operations {
                    load,
                    store: wgpu::StoreOp::Store,
                },
            })],
            ..Default::default()
        });
        pass.set_pipeline(&self.pipeline);
        pass.set_bind_group(0, &self.screen_bind_group, &[]);
        pass.set_vertex_buffer(0, self.quad_buffer.slice(..));
        let mut start = first_instance;
        let group_of = |draw: &Draw| self.entries[draw.texture.index()].group;
        for run in draws.chunk_by(|a, b| group_of(a) == group_of(b)) {
            let end = start + run.len() as u32;
            pass.set_bind_group(1, &self.textures[group_of(&run[0])], &[]);
            pass.draw(0..6, start..end);
            start = end;
        }
    }

    fn post_pass(
        &self,
        encoder: &mut wgpu::CommandEncoder,
        pipeline: &wgpu::RenderPipeline,
        source: &wgpu::BindGroup,
        target: &wgpu::TextureView,
    ) {
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: target,
                depth_slice: None,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                    store: wgpu::StoreOp::Store,
                },
            })],
            ..Default::default()
        });
        pass.set_pipeline(pipeline);
        pass.set_bind_group(0, &self.post.params_group, &[]);
        pass.set_bind_group(1, source, &[]);
        pass.draw(0..3, 0..1);
    }

    pub(crate) fn render(&mut self, clear: wgpu::Color, frame_data: &Frame) {
        let direct = !frame_data.post.vignette_active() && !frame_data.post.blur_active();
        if !direct {
            self.ensure_targets();
        }
        let layers = &frame_data.layers;
        let budget = MAX_QUADS;
        let scene = &layers.scene[..layers.scene.len().min(budget)];
        let hud = &layers.hud[..layers.hud.len().min(budget - scene.len())];
        let overlay = &layers.overlay[..layers.overlay.len().min(budget - scene.len() - hud.len())];
        let mut quads = std::mem::take(&mut self.quad_scratch);
        quads.clear();
        quads.extend(scene.iter().chain(hud).chain(overlay).map(|draw| {
            let mut quad = draw.quad;
            quad.remap_uv(self.entries[draw.texture.index()].region);
            quad
        }));
        self.queue
            .write_buffer(&self.quad_buffer, 0, bytemuck::cast_slice(&quads));
        self.quad_scratch = quads;
        self.queue.write_buffer(
            &self.globals_buffer,
            0,
            bytemuck::bytes_of(&Globals::new(frame_data.camera, &frame_data.lights)),
        );
        self.queue.write_buffer(
            &self.post.params_buffer,
            0,
            bytemuck::bytes_of(&PostUniform::new(
                &frame_data.post,
                self.config.width,
                self.config.height,
            )),
        );
        let frame = match self.surface.get_current_texture() {
            wgpu::CurrentSurfaceTexture::Success(frame)
            | wgpu::CurrentSurfaceTexture::Suboptimal(frame) => frame,
            wgpu::CurrentSurfaceTexture::Outdated | wgpu::CurrentSurfaceTexture::Lost => {
                self.surface.configure(&self.device, &self.config);
                return;
            }
            wgpu::CurrentSurfaceTexture::Occluded => {
                std::thread::sleep(OCCLUDED_WAIT);
                return;
            }
            other => {
                self.skipped_frames += 1;
                if self.skipped_frames % 600 == 1 {
                    tracing::warn!(
                        skipped = self.skipped_frames,
                        "the surface gave no texture: {}",
                        match other {
                            wgpu::CurrentSurfaceTexture::Timeout => "timeout",
                            wgpu::CurrentSurfaceTexture::Validation => "validation",
                            _ => "other",
                        }
                    );
                }
                return;
            }
        };
        let surface_view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor::default());

        if direct {
            self.quads_pass(
                &mut encoder,
                &surface_view,
                wgpu::LoadOp::Clear(clear),
                scene,
                0,
            );
            self.quads_pass(
                &mut encoder,
                &surface_view,
                wgpu::LoadOp::Load,
                hud,
                scene.len() as u32,
            );
            self.quads_pass(
                &mut encoder,
                &surface_view,
                wgpu::LoadOp::Load,
                overlay,
                (scene.len() + hud.len()) as u32,
            );
            self.queue.submit([encoder.finish()]);
            self.queue.present(frame);
            return;
        }
        let Some(targets) = &self.targets else {
            return;
        };

        self.quads_pass(
            &mut encoder,
            &targets.views[0],
            wgpu::LoadOp::Clear(clear),
            scene,
            0,
        );
        let mut current = 0;
        if frame_data.post.vignette_active() {
            self.post_pass(
                &mut encoder,
                &self.post.vignette,
                &targets.groups[current],
                &targets.views[1 - current],
            );
            current = 1 - current;
        }
        self.quads_pass(
            &mut encoder,
            &targets.views[current],
            wgpu::LoadOp::Load,
            hud,
            scene.len() as u32,
        );
        if frame_data.post.blur_active() {
            self.post_pass(
                &mut encoder,
                &self.post.blur,
                &targets.groups[current],
                &targets.views[1 - current],
            );
            current = 1 - current;
        }
        self.post_pass(
            &mut encoder,
            &self.post.blit,
            &targets.groups[current],
            &surface_view,
        );
        self.quads_pass(
            &mut encoder,
            &surface_view,
            wgpu::LoadOp::Load,
            overlay,
            (scene.len() + hud.len()) as u32,
        );
        self.queue.submit([encoder.finish()]);
        self.queue.present(frame);
    }
}

impl Post {
    fn new(
        device: &wgpu::Device,
        format: wgpu::TextureFormat,
        texture_layout: &wgpu::BindGroupLayout,
    ) -> Self {
        let params_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("post params"),
            size: size_of::<PostUniform>() as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let params_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("post params"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });
        let params_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("post params"),
            layout: &params_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: params_buffer.as_entire_binding(),
            }],
        });
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("post"),
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            ..Default::default()
        });
        let shader = device.create_shader_module(wgpu::include_wgsl!("post.wgsl"));
        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("post"),
            bind_group_layouts: &[Some(&params_layout), Some(texture_layout)],
            immediate_size: 0,
        });
        let pipeline = |entry: &str| {
            device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some(entry),
                layout: Some(&layout),
                vertex: wgpu::VertexState {
                    module: &shader,
                    entry_point: Some("vs_fullscreen"),
                    compilation_options: wgpu::PipelineCompilationOptions::default(),
                    buffers: &[],
                },
                fragment: Some(wgpu::FragmentState {
                    module: &shader,
                    entry_point: Some(entry),
                    compilation_options: wgpu::PipelineCompilationOptions::default(),
                    targets: &[Some(format.into())],
                }),
                primitive: wgpu::PrimitiveState::default(),
                depth_stencil: None,
                multisample: wgpu::MultisampleState::default(),
                multiview_mask: None,
                cache: None,
            })
        };
        Self {
            blit: pipeline("fs_blit"),
            vignette: pipeline("fs_vignette"),
            blur: pipeline("fs_blur"),
            params_buffer,
            params_group,
            sampler,
        }
    }
}
