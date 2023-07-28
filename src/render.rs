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

#[derive(Debug, Clone, Copy)]
pub struct Pixel {
    pub x: usize,
    pub y: usize,
    pub color: Rgb,
}

impl Pixel {
    pub fn new(x: usize, y: usize, color: Rgb) -> Self {
        Self { x, y, color }
    }
}

#[derive(Debug, Clone)]
pub struct RenderParams<F: ColorComputer> {
    pub width: usize,
    pub height: usize,
    pub scale: f64,
    pub offset: Complex,
    pub color_computer: F,
}

#[derive(Debug)]
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

    pub fn render_pixels<F: ColorComputer>(
        &self,
        params: RenderParams<F>,
    ) -> impl Iterator<Item = Pixel> {
        let coords = (0..params.height).cartesian_product(0..params.width);

        // Rows are divided into chunks to be rendered by a thread pool.
        // Grouping by row makes the result compatible with the memory layout of the window buffer
        // In the end, all task results are collected and sorted by row index.
        let chunk_size = (params.height / self.pool_size) * params.width;
        let handles: Vec<_> = coords
            .chunks(chunk_size)
            .into_iter()
            .map(|chunk| chunk.collect())
            .map(move |coords: Vec<_>| {
                self.thread_pool
                    .spawn_with_handle(self.render_task(coords, params.clone()))
                    .expect("Failed to spawn render task")
                    .into_stream()
            })
            .collect();

        block_on_stream(futures::stream::select_all(handles))
            // Results are computed asynchronously and will appear in non-sequential order.
            // Since task distribution between workers is done based on row index (y coordinate),
            // we can sort the vectors by using the y coordinate of the first element.
            .sorted_by_key(|v| v.first().map(|p| p.y))
            .flatten()
    }

    fn render_task<F: ColorComputer>(
        &self,
        coords: Vec<(usize, usize)>,
        params: RenderParams<F>,
    ) -> impl Future<Output = Vec<Pixel>> {
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
            coords
                .into_iter()
                // Bind as (y, x) because the cartesian product is (0..height) X (0..width)
                .map(|(y, x)| {
                    let zx = scale * (inv_width * x as f64 - 0.5);
                    let zy = scale * (inv_height * y as f64 - 0.5);
                    let z = Complex::new(zx, zy);
                    Pixel::new(x, y, color_computer(z - offset))
                })
                .collect::<Vec<_>>()
        }
    }
}
