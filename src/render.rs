use crate::{
    color::Rgb,
    simd::{Array, SimdComplex},
    utils::{Complex, FnSync},
    view::ComplexPlaneView,
};
use itertools::Itertools;
use rayon::iter::{IndexedParallelIterator, IntoParallelIterator, ParallelIterator};
use std::{num::NonZeroUsize, ops::Range, usize};

pub struct Renderer {
    chunk_count: usize,
}

impl Renderer {
    const DEFAULT_CHUNK_COUNT: usize = 16;

    pub fn new() -> Self {
        Self {
            chunk_count: Self::resolve_chunk_count(),
        }
    }

    fn resolve_chunk_count() -> usize {
        std::thread::available_parallelism()
            .map(NonZeroUsize::get)
            .unwrap_or(Self::DEFAULT_CHUNK_COUNT)
    }

    pub fn render<F>(&self, view: &ComplexPlaneView, color_computer: F) -> impl Iterator<Item = Rgb>
    where
        F: FnSync(SimdComplex) -> Array<Rgb>,
    {
        let view_height = view.height();
        let chunk_height = view_height.div_ceil(self.chunk_count);

        let mut chunks: Vec<_> = Vec::with_capacity(self.chunk_count);
        (0..view_height)
            .step_by(chunk_height)
            // Collecting here before converting to a parallel iterator allows 
            // using the order-preserving `collect_into_vec` later.
            // Otherwise, we would need to sort the chunks by row index.
            .collect_vec()
            .into_par_iter()
            .map(move |chunk_start| {
                let chunk_end = (chunk_start + chunk_height).min(view_height);
                self.simd_render_chunk(chunk_start..chunk_end, view, &color_computer)
            })
            .collect_into_vec(&mut chunks);
        chunks.into_iter().flatten()
    }

    fn simd_render_chunk<F>(
        &self,
        chunk_rows: Range<usize>,
        view: &ComplexPlaneView,
        color_computer: F,
    ) -> Vec<Rgb>
    where
        F: FnSync(SimdComplex) -> Array<Rgb>,
    {
        let view_width = view.width();
        let chunk_height = chunk_rows.end - chunk_rows.start;
        let chunk_size = chunk_height * view_width;
        let pixel_to_complex = view.pixel_mapper();
        chunk_rows
            .cartesian_product(0..view_width)
            .map(|(y, x)| pixel_to_complex(x, y))
            .chunks(SimdComplex::LEN)
            .into_iter()
            .map(|chunk| Self::chunk_to_simd_complex(chunk))
            .flat_map(color_computer)
            .take(chunk_size)
            .collect_vec()
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
