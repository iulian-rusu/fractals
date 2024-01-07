use std::{ops::Range, usize};

use crate::{
    color::Rgb,
    shared::{ColorComputer, Complex},
};
use futures::{
    executor::{block_on_stream, ThreadPool},
    task::SpawnExt,
    Future, FutureExt,
};
use itertools::Itertools;

struct RenderedChunk {
    pub pixels: Vec<Rgb>,
    pub start_row: usize,
}

#[derive(Debug, Clone)]
pub struct RenderParams<F: ColorComputer(Complex) -> Rgb> {
    pub width: usize,
    pub height: usize,
    pub scale: f64,
    pub offset: Complex,
    pub color_computer: F,
}

pub struct Renderer {
    thread_pool: ThreadPool,
    pool_size: usize,
}

impl Renderer {
    pub fn new(pool_size: usize) -> Self {
        let thread_pool = ThreadPool::builder()
            .pool_size(pool_size)
            .create()
            .expect("Failed to create render thread pool");
        Self {
            thread_pool,
            pool_size,
        }
    }

    pub fn render_pixels<F: ColorComputer(Complex) -> Rgb>(
        &self,
        params: RenderParams<F>,
    ) -> impl Iterator<Item = Rgb> {
        // Rows are divided into chunks to be rendered by a thread pool.
        // Grouping by row makes the result compatible with the memory layout of the window buffer
        // In the end, all task results are collected and sorted by row index.
        let window_height = params.height;
        let chunk_height = window_height.div_ceil(self.pool_size);
        let handles: Vec<_> = (0..window_height)
            .step_by(chunk_height)
            .map(|chunk_start| {
                let chunk_end = (chunk_start + chunk_height).min(window_height);
                self.thread_pool
                    .spawn_with_handle(self.render_chunk(chunk_start..chunk_end, params.clone()))
                    .expect("Failed to spawn render task")
                    .into_stream()
            })
            .collect();

        block_on_stream(futures::stream::select_all(handles))
            // Results must be sorted because each chunk is rendered in non-deterministic order.
            .sorted_by_key(|chunk| chunk.start_row)
            .map(|chunk| chunk.pixels)
            .flatten()
    }

    fn render_chunk<F: ColorComputer(Complex) -> Rgb>(
        &self,
        chunk_rows: Range<usize>,
        params: RenderParams<F>,
    ) -> impl Future<Output = RenderedChunk> {
        let start_row = chunk_rows.start;
        let RenderParams {
            width,
            height,
            scale,
            offset,
            color_computer,
        } = params;

        async move {
            let inv_width = 1.0 / width as f64;
            let inv_height = 1.0 / height as f64;
            let pixels = chunk_rows
                .cartesian_product(0..width)
                .into_iter()
                // Bind as (y, x) because the cartesian product is (0..height) X (0..width)
                .map(|(y, x)| {
                    let re = scale * (inv_width * x as f64 - 0.5);
                    let im = scale * (inv_height * y as f64 - 0.5);
                    let z = Complex::new(re, im);
                    color_computer(z - offset)
                })
                .collect::<Vec<_>>();

            RenderedChunk { pixels, start_row }
        }
    }
}
