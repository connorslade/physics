use std::{mem, sync::Arc};

use bytemuck::{Pod, Zeroable};
use once_cell::sync::Lazy;
use vello::{
    kurbo::{Affine, Circle},
    peniko::{Blob, Color, Fill, Font},
    skrifa::{prelude::Size, FontRef, MetadataProvider},
    Glyph, Scene,
};
use wgpu::{
    util::{BufferInitDescriptor, DeviceExt},
    BindGroup, BindGroupDescriptor, BindGroupLayoutDescriptor, Buffer, BufferAddress, BufferUsages,
    ColorTargetState, ColorWrites, CommandEncoder, Device, Face, FragmentState, IndexFormat,
    LoadOp, MultisampleState, Operations, PipelineCompilationOptions, PipelineLayoutDescriptor,
    PrimitiveState, RenderPassColorAttachment, RenderPassDescriptor, RenderPipeline,
    RenderPipelineDescriptor, ShaderModuleDescriptor, ShaderSource, StoreOp, TextureView,
    VertexAttribute, VertexBufferLayout, VertexFormat, VertexState, VertexStepMode,
};

use crate::world::World;

use super::{PARTICLE_RADIUS, TEXTURE_FORMAT};

static FONT: Lazy<Font> = Lazy::new(|| {
    let font_data = include_bytes!("../../assets/JetBrainsMono-Regular.ttf");
    Font::new(Blob::new(Arc::new(font_data)), 0)
});

static FONT_REF: Lazy<FontRef> = Lazy::new(|| FontRef::new(FONT.data.data()).unwrap());

pub struct FieldLinePipeline {
    pipeline: RenderPipeline,
    bind_group: BindGroup,
    vertex_buf: Buffer,
    index_buf: Buffer,
}

#[derive(Copy, Clone, Pod, Zeroable)]
#[repr(C)]
struct Vertex {
    position: [f32; 4],
    tex_coords: [f32; 2],
}

impl Vertex {
    const fn new(position: [f32; 4], tex_coords: [f32; 2]) -> Self {
        Self {
            position,
            tex_coords,
        }
    }
}

pub fn draw(scale: f32, scene: &mut Scene, world: &mut World) {
    let font_size = 16.0 * scale;

    for (pos, charge) in &world.particles {
        let color = if *charge > 0 { Color::RED } else { Color::BLUE };
        let shape = Circle::new((pos.x, pos.y), (scale * PARTICLE_RADIUS) as f64);
        scene.fill(Fill::NonZero, Affine::IDENTITY, color, None, &shape);

        let text = format!(
            "{}{}",
            if *charge > 0 { "+" } else { "-" },
            charge.unsigned_abs()
        );

        let axes = FONT_REF.axes();
        let variations: &[(&str, f32)] = &[];
        let var_loc = axes.location(variations);
        let metrics = FONT_REF.metrics(Size::new(font_size), &var_loc);
        let glyph_metrics = FONT_REF.glyph_metrics(Size::new(font_size), &var_loc);

        let text_height = metrics.ascent + metrics.descent;
        let text_width = text.chars().fold(0.0, |acc, x| {
            acc + glyph_metrics
                .advance_width(FONT_REF.charmap().map(x).unwrap_or_default())
                .unwrap_or_default()
        });

        let mut x_offset = -text_width / 2.0;

        scene
            .draw_glyphs(&FONT)
            .font_size(font_size)
            .brush(Color::WHITE)
            .draw(
                Fill::EvenOdd,
                text.chars().map(|x| {
                    let gid = FONT_REF.charmap().map(x).unwrap_or_default();
                    let (x, y) = (pos.x + x_offset, pos.y + text_height / 2.0);
                    x_offset += glyph_metrics.advance_width(gid).unwrap_or_default();
                    Glyph {
                        id: gid.to_u32(),
                        x,
                        y,
                    }
                }),
            );
    }
}

const VERTEX_DATA: [Vertex; 4] = [
    Vertex::new([-1.0, -1.0, 1.0, 1.0], [0.0, 0.0]),
    Vertex::new([1.0, -1.0, 1.0, 1.0], [1.0, 0.0]),
    Vertex::new([1.0, 1.0, 1.0, 1.0], [1.0, 1.0]),
    Vertex::new([-1.0, 1.0, 1.0, 1.0], [0.0, 1.0]),
];

const INDEX_DATA: &[u16] = &[0, 1, 2, 2, 3, 0];

impl FieldLinePipeline {
    pub fn new(device: &Device) -> Self {
        let vertex_buf = device.create_buffer_init(&BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&VERTEX_DATA),
            usage: BufferUsages::VERTEX,
        });
        let index_buf = device.create_buffer_init(&BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(INDEX_DATA),
            usage: BufferUsages::INDEX,
        });

        let render_shader = device.create_shader_module(ShaderModuleDescriptor {
            label: None,
            source: ShaderSource::Wgsl(include_str!("shaders/render.wgsl").into()),
        });

        let bind_group_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: None,
            entries: &[],
        });

        let bind_group = device.create_bind_group(&BindGroupDescriptor {
            label: None,
            layout: &bind_group_layout,
            entries: &[],
        });

        let vertex_buffers = [VertexBufferLayout {
            array_stride: mem::size_of::<Vertex>() as BufferAddress,
            step_mode: VertexStepMode::Vertex,
            attributes: &[
                VertexAttribute {
                    format: VertexFormat::Float32x4,
                    offset: 0,
                    shader_location: 0,
                },
                VertexAttribute {
                    format: VertexFormat::Float32x2,
                    offset: 4 * 4,
                    shader_location: 1,
                },
            ],
        }];

        let pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });

        let pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
            label: None,
            layout: Some(&pipeline_layout),
            vertex: VertexState {
                module: &render_shader,
                entry_point: "vert",
                buffers: &vertex_buffers,
                compilation_options: PipelineCompilationOptions::default(),
            },
            fragment: Some(FragmentState {
                module: &render_shader,
                entry_point: "frag",
                targets: &[Some(ColorTargetState {
                    format: TEXTURE_FORMAT,
                    blend: None,
                    write_mask: ColorWrites::all(),
                })],
                compilation_options: PipelineCompilationOptions::default(),
            }),
            primitive: PrimitiveState {
                cull_mode: Some(Face::Back),
                ..Default::default()
            },
            depth_stencil: None,
            multiview: None,
            multisample: MultisampleState::default(),
            cache: None,
        });

        Self {
            pipeline,
            bind_group,
            vertex_buf,
            index_buf,
        }
    }

    pub fn draw(&mut self, encoder: &mut CommandEncoder, view: &TextureView) {
        let mut render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
            label: None,
            color_attachments: &[Some(RenderPassColorAttachment {
                view,
                resolve_target: None,
                ops: Operations {
                    load: LoadOp::Load,
                    store: StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });
        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_bind_group(0, &self.bind_group, &[]);
        render_pass.set_index_buffer(self.index_buf.slice(..), IndexFormat::Uint16);
        render_pass.set_vertex_buffer(0, self.vertex_buf.slice(..));
        render_pass.draw_indexed(0..6, 0, 0..1);
    }
}
