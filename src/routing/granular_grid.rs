use crate::flatbuffer::TransitNode;
use std::iter;
use std::ops::{Index, IndexMut};
use std::ops;
use crate::Bounds;

pub struct GranularGrid<T> {
    rows: u32,
    cols: u32,
    bounds: Bounds,
    cells: Vec<Vec<T>>,
}

impl<T> GranularGrid<T> {
    pub fn new(rows: u32, cols: u32, bounds: &Bounds) -> Self {
        let size = (rows as usize).checked_mul(cols as usize).expect("too big");

        let cells = iter::repeat_with(|| Vec::<T>::new()).take(size).collect();
        Self {
            rows,
            cols,
            bounds: bounds.to_owned(),
            cells,
        }
    }
//     TODO is fn get_int_index necessary
}

impl<T> Index<[f32; 2]> for GranularGrid<T> {
    type Output = Vec<T>;

    fn index(&self, idx: [f32; 2]) -> &Self::Output {
        let norm_x = ((idx[0] / self.bounds.max().x()) * self.cols as f32) as usize;
        let norm_y = ((idx[1] / self.bounds.max().y()) * self.rows as f32) as usize;

        &self.cells[norm_x * self.cols as usize + norm_y]
    }
}

impl<T> IndexMut<[f32; 2]> for GranularGrid<T> {
    fn index_mut(&mut self, idx: [f32; 2]) -> &mut Self::Output {
        let norm_x = (idx[0] / self.bounds.max().x()) as usize;
        let norm_y = (idx[1] / self.bounds.max().y()) as usize;
        &mut self.cells[norm_x * self.cols as usize + norm_y]
    }
}