use std::time::Instant;

use anyhow::{Ok, Result};
use compute::{
    export::{
        wgpu::{include_wgsl, ShaderStages},
        winit::window::WindowAttributes,
    },
    gpu::Gpu,
};

mod app;
mod types;
use crate::types::Uniform;
use app::App;

fn main() -> Result<()> {
    let gpu = Gpu::init()?;

    let ctx = Uniform::default();
    let uniform = gpu.create_uniform(&ctx)?;
    let dots = gpu.create_storage(Vec::new())?;

    let render = gpu
        .render_pipeline(include_wgsl!("../shaders/render.wgsl"))
        .bind_buffer(&dots, ShaderStages::VERTEX_FRAGMENT)
        .bind_buffer(&uniform, ShaderStages::VERTEX_FRAGMENT)
        .finish();
    let compute = gpu
        .compute_pipeline(include_wgsl!("../shaders/compute.wgsl"))
        .bind_buffer(&dots)
        .bind_buffer(&uniform)
        .finish();

    gpu.create_window(
        WindowAttributes::default().with_title("Dots Example"),
        App {
            render,
            compute,

            ctx,
            uniform,
            dots,

            last_frame: Instant::now(),
        },
    )
    .run()?;

    Ok(())
}
