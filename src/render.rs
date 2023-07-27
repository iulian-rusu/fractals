use crate::{color::Rgb, shared::Complex};
use futures::{
    executor::{block_on_stream, ThreadPool},
    task::SpawnExt,
    FutureExt,
};
use itertools::Itertools;

#[derive(Debug, Clone, Copy)]
pub struct RenderConfig<'a> {
    pub scale: f64,
    pub width: usize,
    pub height: usize,
    pub thread_pool: &'a ThreadPool,
    pub pool_size: usize,
}

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

pub fn render_pixels<'a, F>(
    render_config: RenderConfig<'a>,
    color_computer: F,
) -> impl Iterator<Item = Pixel>
where
    F: Fn(Complex) -> Rgb + Send + Clone + 'static,
{
    let RenderConfig {
        scale,
        width,
        height,
        thread_pool,
        pool_size,
    } = render_config;

    let scale_x = scale / width as f64;
    let scale_y = scale / height as f64;
    let half_scale = scale / 2.0;
    let coords = (0..height).cartesian_product(0..width);

    // Rows are divided into chunks to be rendered by a thrad pool.
    // Grouping by row makes the result compatible with the memory layout of the window buffer
    // At the end, all task results are collected and sorted by row index.
    let chunk_size = (height / pool_size) * width;
    let handles: Vec<_> = coords
        .chunks(chunk_size)
        .into_iter()
        .map(|chunk| chunk.collect())
        .map(move |coords: Vec<_>| {
            thread_pool
                .spawn_with_handle({
                    let color_computer = color_computer.clone();
                    async move {
                        coords
                            .into_iter()
                            // Bind as (y, x) because the cartesian product is (0..height) X (0..width)
                            .map(|(y, x)| {
                                let zx = scale_x * x as f64 - half_scale;
                                let zy = scale_y * y as f64 - half_scale;
                                let z = Complex::new(zx, zy);
                                Pixel::new(x, y, color_computer(z))
                            })
                            .collect::<Vec<_>>()
                    }
                })
                .expect("Failed to spawn task")
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
