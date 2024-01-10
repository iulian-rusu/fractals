use crate::{
    color::Rgb,
    shared::{ColorComputer, Complex},
    viewport::Viewport,
};
use futures::{
    executor::{block_on_stream, ThreadPool},
    task::SpawnExt,
    Future, FutureExt,
};
use itertools::Itertools;
use std::{ops::Range, usize};

struct RenderedChunk {
    pub pixels: Vec<Rgb>,
    pub start_row: usize,
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

    pub fn render<F: ColorComputer(Complex) -> Rgb>(
        &self,
        viewport: &Viewport,
        color_computer: F,
    ) -> impl Iterator<Item = Rgb> {
        // Rows are divided into chunks to be rendered by a thread pool.
        // Grouping by row makes the result compatible with the memory layout of the window buffer
        // In the end, all task results are collected and sorted by row index.
        let viewport_height = viewport.height();
        let chunk_height = viewport_height.div_ceil(self.pool_size);
        let handles: Vec<_> = (0..viewport_height)
            .step_by(chunk_height)
            .map(|chunk_start| {
                let chunk_end = (chunk_start + chunk_height).min(viewport_height);
                self.thread_pool
                    .spawn_with_handle(self.render_chunk(
                        chunk_start..chunk_end,
                        viewport,
                        color_computer.clone(),
                    ))
                    .expect("Spawning a render task should not fail")
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
        viewport: &Viewport,
        color_computer: F,
    ) -> impl Future<Output = RenderedChunk> {
        let start_row = chunk_rows.start;
        let viewport_width = viewport.width();
        let viewport_mapper = viewport.mapper();
        async move {
            let pixels: Vec<Rgb> = chunk_rows
                .cartesian_product(0..viewport_width)
                .into_iter()
                // Bind as (y, x) because the cartesian product is (0..height) X (0..width)
                .map(|(y, x)| viewport_mapper(x, y))
                .map(|z| color_computer(z))
                .collect::<Vec<_>>();

            RenderedChunk { pixels, start_row }
        }
    }
}
