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
use super::{Draw, Frame, Occluders, Quad, TextureId};
use crate::viewport::TARGET;

const INITIAL_QUADS: usize = 10_000;
const MAX_QUADS: usize = 1 << 20;
const LIGHTMAP_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Rgba16Float;
const LIGHTMAP_SCALE: f32 = 0.5;
/// Headless frames are plain RGBA, so they can be saved as they are.
const OFFSCREEN_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Rgba8UnormSrgb;
const OCCLUDED_WAIT: std::time::Duration = std::time::Duration::from_millis(16);

#[derive(Debug)]
pub(crate) struct Gpu {
    /// The window's surface; `None` when rendering headless into [`Gpu::offscreen`].
    surface: Option<wgpu::Surface<'static>>,
    offscreen: Option<wgpu::Texture>,
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
    quad_capacity: usize,
    trimmed_frames: u32,
    post: Post,
    targets: Option<Targets>,
    light: Lighting,
}

/// Renders the light of the visible area, walls casting shadows, into a small texture that the
/// quads read their light from.
struct Lighting {
    pipeline: wgpu::RenderPipeline,
    layout: wgpu::BindGroupLayout,
    group: wgpu::BindGroup,
    lightmap: wgpu::TextureView,
    occluders: u64,
}

impl std::fmt::Debug for Lighting {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Lighting")
            .field("occluders", &self.occluders)
            .finish()
    }
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
    slow: wgpu::RenderPipeline,
    blur_horizontal: wgpu::RenderPipeline,
    blur_vertical: wgpu::RenderPipeline,
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
        Ok(Self::build(Some(surface), None, device, queue, config))
    }

    /// Renders into an offscreen texture of the given size instead of a window, for screenshots
    /// without opening anything. Read a frame back with [`Gpu::capture`].
    pub(crate) fn headless(width: u32, height: u32) -> Result<Self, Box<dyn Error>> {
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::new_without_display_handle());
        let adapter =
            pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions::default()))?;
        let (device, queue) =
            pollster::block_on(adapter.request_device(&wgpu::DeviceDescriptor::default()))?;
        let format = OFFSCREEN_FORMAT;
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format,
            width: width.max(1),
            height: height.max(1),
            present_mode: wgpu::PresentMode::Fifo,
            desired_maximum_frame_latency: 2,
            alpha_mode: wgpu::CompositeAlphaMode::Opaque,
            view_formats: Vec::new(),
            color_space: wgpu::SurfaceColorSpace::default(),
        };
        let offscreen = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("offscreen"),
            size: wgpu::Extent3d {
                width: config.width,
                height: config.height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC,
            view_formats: &[],
        });
        Ok(Self::build(None, Some(offscreen), device, queue, config))
    }

    fn build(
        surface: Option<wgpu::Surface<'static>>,
        offscreen: Option<wgpu::Texture>,
        device: wgpu::Device,
        queue: wgpu::Queue,
        config: wgpu::SurfaceConfiguration,
    ) -> Self {
        let globals_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("globals"),
            contents: bytemuck::bytes_of(&Globals::new(glam::Vec2::ZERO, &[], None)),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });
        let light = Lighting::new(&device, &queue, &globals_buffer);
        let lightmap_sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("lightmap"),
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });
        let screen_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("screen"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
        });
        let screen_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("screen"),
            layout: &screen_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: globals_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::TextureView(&light.lightmap),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::Sampler(&lightmap_sampler),
                },
            ],
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
        let quad_buffer = quad_buffer(&device, INITIAL_QUADS);
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
            offscreen,
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
            quad_capacity: INITIAL_QUADS,
            trimmed_frames: 0,
            post,
            targets: None,
            light,
        };
        gpu.upload(&Pixels::white());
        gpu
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
        if let Some(surface) = &self.surface {
            surface.configure(&self.device, &self.config);
        }
        tracing::info!(present_mode = ?self.config.present_mode, "present mode set");
    }

    pub(crate) fn resize(&mut self, size: PhysicalSize<u32>) {
        if size.width == 0 || size.height == 0 {
            return;
        }
        let Some(surface) = &self.surface else {
            return;
        };
        self.config.width = size.width;
        self.config.height = size.height;
        surface.configure(&self.device, &self.config);
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

    fn reserve_quads(&mut self, scene: usize, hud: usize, overlay: usize) -> (usize, usize, usize) {
        let wanted = scene + hud + overlay;
        if wanted > self.quad_capacity && self.quad_capacity < MAX_QUADS {
            self.quad_capacity = wanted.next_power_of_two().min(MAX_QUADS);
            self.quad_buffer = quad_buffer(&self.device, self.quad_capacity);
            tracing::info!(capacity = self.quad_capacity, "grew the quad buffer");
        }
        let fitted = fit_layers(scene, hud, overlay, self.quad_capacity);
        if fitted.0 < scene {
            self.trimmed_frames += 1;
            if self.trimmed_frames % 600 == 1 {
                tracing::warn!(
                    wanted,
                    capacity = self.quad_capacity,
                    "too many quads, the topmost scene quads are not drawn"
                );
            }
        }
        fitted
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
        let direct = !frame_data.post.vignette_active()
            && !frame_data.post.blur_active()
            && !frame_data.post.time_slow_active();
        if !direct {
            self.ensure_targets();
        }
        let layers = &frame_data.layers;
        let (scene_count, hud_count, overlay_count) =
            self.reserve_quads(layers.scene.len(), layers.hud.len(), layers.overlay.len());
        let scene = &layers.scene[..scene_count];
        let hud = &layers.hud[..hud_count];
        let overlay = &layers.overlay[..overlay_count];
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
            bytemuck::bytes_of(&Globals::new(
                frame_data.camera,
                &frame_data.lights,
                frame_data.occluders.as_deref(),
            )),
        );
        if let Some(occluders) = &frame_data.occluders {
            self.light
                .set_occluders(&self.device, &self.queue, &self.globals_buffer, occluders);
        }
        self.queue.write_buffer(
            &self.post.params_buffer,
            0,
            bytemuck::bytes_of(&PostUniform::new(
                &frame_data.post,
                self.config.width,
                self.config.height,
            )),
        );
        let (surface_view, frame) = match (&self.surface, &self.offscreen) {
            (Some(_), _) => {
                let Some(frame) = self.next_surface_texture() else {
                    return;
                };
                let view = frame
                    .texture
                    .create_view(&wgpu::TextureViewDescriptor::default());
                (view, Some(frame))
            }
            (None, Some(offscreen)) => (
                offscreen.create_view(&wgpu::TextureViewDescriptor::default()),
                None,
            ),
            (None, None) => return,
        };
        let present = |queue: &wgpu::Queue, frame: Option<wgpu::SurfaceTexture>| {
            if let Some(frame) = frame {
                queue.present(frame);
            }
        };
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor::default());
        self.light.draw(&mut encoder);

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
            present(&self.queue, frame);
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
        if frame_data.post.time_slow_active() {
            self.post_pass(
                &mut encoder,
                &self.post.slow,
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
            for pipeline in [&self.post.blur_horizontal, &self.post.blur_vertical] {
                self.post_pass(
                    &mut encoder,
                    pipeline,
                    &targets.groups[current],
                    &targets.views[1 - current],
                );
                current = 1 - current;
            }
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
        present(&self.queue, frame);
    }

    fn next_surface_texture(&mut self) -> Option<wgpu::SurfaceTexture> {
        let surface = self.surface.as_ref()?;
        match surface.get_current_texture() {
            wgpu::CurrentSurfaceTexture::Success(frame)
            | wgpu::CurrentSurfaceTexture::Suboptimal(frame) => Some(frame),
            wgpu::CurrentSurfaceTexture::Outdated | wgpu::CurrentSurfaceTexture::Lost => {
                surface.configure(&self.device, &self.config);
                None
            }
            wgpu::CurrentSurfaceTexture::Occluded => {
                std::thread::sleep(OCCLUDED_WAIT);
                None
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
                None
            }
        }
    }

    /// Reads the last headless frame back as RGBA pixels: `(width, height, rgba)`.
    pub(crate) fn capture(&self) -> Result<(u32, u32, Vec<u8>), Box<dyn Error>> {
        let texture = self
            .offscreen
            .as_ref()
            .ok_or("only a headless renderer can capture")?;
        let (width, height) = (self.config.width, self.config.height);
        let row = 4 * width;
        let padded =
            row.div_ceil(wgpu::COPY_BYTES_PER_ROW_ALIGNMENT) * wgpu::COPY_BYTES_PER_ROW_ALIGNMENT;
        let buffer = self.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("capture"),
            size: u64::from(padded) * u64::from(height),
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
            mapped_at_creation: false,
        });
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor::default());
        encoder.copy_texture_to_buffer(
            texture.as_image_copy(),
            wgpu::TexelCopyBufferInfo {
                buffer: &buffer,
                layout: wgpu::TexelCopyBufferLayout {
                    offset: 0,
                    bytes_per_row: Some(padded),
                    rows_per_image: Some(height),
                },
            },
            texture.size(),
        );
        self.queue.submit([encoder.finish()]);
        let slice = buffer.slice(..);
        let (sender, receiver) = std::sync::mpsc::channel();
        slice.map_async(wgpu::MapMode::Read, move |result| {
            let _ = sender.send(result);
        });
        self.device.poll(wgpu::PollType::wait_indefinitely())?;
        receiver.recv()??;
        let mapped = slice.get_mapped_range()?;
        let mut rgba = Vec::with_capacity((row * height) as usize);
        for line in mapped.chunks(padded as usize) {
            rgba.extend_from_slice(&line[..row as usize]);
        }
        drop(mapped);
        buffer.unmap();
        Ok((width, height, rgba))
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
            slow: pipeline("fs_slow"),
            blur_horizontal: pipeline("fs_blur_horizontal"),
            blur_vertical: pipeline("fs_blur_vertical"),
            params_buffer,
            params_group,
            sampler,
        }
    }
}

impl Lighting {
    fn new(device: &wgpu::Device, queue: &wgpu::Queue, globals: &wgpu::Buffer) -> Self {
        let size = (TARGET * LIGHTMAP_SCALE).as_uvec2();
        let lightmap = device
            .create_texture(&wgpu::TextureDescriptor {
                label: Some("lightmap"),
                size: wgpu::Extent3d {
                    width: size.x,
                    height: size.y,
                    depth_or_array_layers: 1,
                },
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: LIGHTMAP_FORMAT,
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT
                    | wgpu::TextureUsages::TEXTURE_BINDING,
                view_formats: &[],
            })
            .create_view(&wgpu::TextureViewDescriptor::default());
        let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("light"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: false },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
            ],
        });
        let shader = device.create_shader_module(wgpu::include_wgsl!("lightmap.wgsl"));
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("light"),
            bind_group_layouts: &[Some(&layout)],
            immediate_size: 0,
        });
        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("light"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_fullscreen"),
                compilation_options: wgpu::PipelineCompilationOptions::default(),
                buffers: &[],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_light"),
                compilation_options: wgpu::PipelineCompilationOptions::default(),
                targets: &[Some(LIGHTMAP_FORMAT.into())],
            }),
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview_mask: None,
            cache: None,
        });
        let empty = occluder_texture(device, queue, (1, 1), &[0]);
        let group = light_group(device, &layout, globals, &empty);
        Self {
            pipeline,
            layout,
            group,
            lightmap,
            occluders: 0,
        }
    }

    fn set_occluders(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        globals: &wgpu::Buffer,
        occluders: &Occluders,
    ) {
        if self.occluders == occluders.id() {
            return;
        }
        let texture = occluder_texture(device, queue, occluders.size(), occluders.cells());
        self.group = light_group(device, &self.layout, globals, &texture);
        self.occluders = occluders.id();
    }

    fn draw(&self, encoder: &mut wgpu::CommandEncoder) {
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("light"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &self.lightmap,
                depth_slice: None,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                    store: wgpu::StoreOp::Store,
                },
            })],
            ..Default::default()
        });
        pass.set_pipeline(&self.pipeline);
        pass.set_bind_group(0, &self.group, &[]);
        pass.draw(0..3, 0..1);
    }
}

fn occluder_texture(
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    (columns, rows): (u32, u32),
    cells: &[u8],
) -> wgpu::TextureView {
    let size = wgpu::Extent3d {
        width: columns.max(1),
        height: rows.max(1),
        depth_or_array_layers: 1,
    };
    let texture = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("occluders"),
        size,
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::R8Unorm,
        usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
        view_formats: &[],
    });
    if cells.len() as u32 == size.width * size.height {
        queue.write_texture(
            texture.as_image_copy(),
            cells,
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(size.width),
                rows_per_image: Some(size.height),
            },
            size,
        );
    }
    texture.create_view(&wgpu::TextureViewDescriptor::default())
}

fn light_group(
    device: &wgpu::Device,
    layout: &wgpu::BindGroupLayout,
    globals: &wgpu::Buffer,
    occluders: &wgpu::TextureView,
) -> wgpu::BindGroup {
    device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("light"),
        layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: globals.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: wgpu::BindingResource::TextureView(occluders),
            },
        ],
    })
}

fn quad_buffer(device: &wgpu::Device, capacity: usize) -> wgpu::Buffer {
    device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("quads"),
        size: (capacity * size_of::<Quad>()) as u64,
        usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    })
}

/// How many quads of each layer fit into the budget: the overlay (cursor, fades) and the HUD
/// are kept whole, the scene gives up its topmost quads first.
fn fit_layers(scene: usize, hud: usize, overlay: usize, budget: usize) -> (usize, usize, usize) {
    let overlay = overlay.min(budget);
    let hud = hud.min(budget - overlay);
    let scene = scene.min(budget - overlay - hud);
    (scene, hud, overlay)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn everything_fits_when_there_is_room() {
        assert_eq!(fit_layers(10, 5, 1, 100), (10, 5, 1));
    }

    #[test]
    fn the_scene_is_trimmed_before_the_hud_and_the_cursor() {
        assert_eq!(fit_layers(100, 5, 1, 50), (44, 5, 1));
        assert_eq!(fit_layers(100, 60, 1, 50), (0, 49, 1));
        assert_eq!(fit_layers(100, 60, 80, 50), (0, 0, 50));
    }
}
