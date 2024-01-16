use crate::{
    color::Rgb,
    simd::{Array, SimdComplex},
    utils::{Complex, FnSend},
    view::ComplexPlaneView,
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

    pub fn render<F>(&self, view: &ComplexPlaneView, color_computer: F) -> impl Iterator<Item = Rgb>
    where
        F: FnSend(SimdComplex) -> Array<Rgb> + Clone,
    {
        // Rows are divided into chunks to be rendered by a thread pool.
        // Grouping by row makes the result compatible with the memory layout of the window buffer
        // In the end, all task results are collected and sorted by row index.
        let view_height = view.height();
        let chunk_height = view_height.div_ceil(self.pool_size);
        let handles: Vec<_> = (0..view_height)
            .step_by(chunk_height)
            .map(|chunk_start| {
                let chunk_end = (chunk_start + chunk_height).min(view_height);
                self.thread_pool
                    .spawn_with_handle(self.simd_render_chunk(
                        chunk_start..chunk_end,
                        view,
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

    fn simd_render_chunk<F>(
        &self,
        chunk_rows: Range<usize>,
        view: &ComplexPlaneView,
        color_computer: F,
    ) -> impl Future<Output = RenderedChunk>
    where
        F: FnSend(SimdComplex) -> Array<Rgb>,
    {
        let view_width = view.width();
        let start_row = chunk_rows.start;
        let chunk_height = chunk_rows.end - chunk_rows.start;
        let chunk_size = chunk_height * view_width;
        let pixel_to_complex = view.pixel_mapper();
        async move {
            let pixels: Vec<Rgb> = chunk_rows
                .cartesian_product(0..view_width)
                .map(|(y, x)| pixel_to_complex(x, y))
                .chunks(SimdComplex::LANES)
                .into_iter()
                .map(|chunk| Self::chunk_to_simd_complex(chunk))
                .map(color_computer)
                .flatten()
                .take(chunk_size)
                .collect();

            RenderedChunk { pixels, start_row }
        }
    }

    fn chunk_to_simd_complex(chunk: impl Iterator<Item = Complex>) -> SimdComplex {
        let mut res = SimdComplex::default();
        for (i, z) in chunk.enumerate() {
            res.re[i] = z.re;
            res.im[i] = z.im;
        }
        res
    }
}
