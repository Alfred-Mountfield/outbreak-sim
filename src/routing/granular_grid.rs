use crate::flatbuffer::TransitNode;
use std::iter;
use std::ops::{Index, IndexMut};
use std::ops;
use crate::Bounds;

pub struct GranularGrid<T> {
    pub rows: u32,
    pub cols: u32,
    pub ratio: f32,
    pub bounds: Bounds,
    cells: Vec<Vec<T>>,
}

impl<T> GranularGrid<T> {
    pub fn new(rows: u32, bounds: &Bounds) -> Self {
        let ratio =  rows as f32 / bounds.max().x() as f32;
        let cols = (bounds.max().y() * ratio) as u32;
        let size = (rows as usize).checked_mul(cols as usize).expect("too big");

        let cells = iter::repeat_with(|| Vec::<T>::new()).take(size).collect();
        Self {
            rows,
            cols,
            ratio,
            bounds: bounds.to_owned(),
            cells,
        }
    }

    // TODO is fn get_int_index necessary
    pub fn get_int_index(&self, col: u32, row: u32) -> &Vec<T> {
        &self.cells[row as usize * self.cols as usize + col as usize]
    }

}

impl<T> Index<[f32; 2]> for GranularGrid<T> {
    type Output = Vec<T>;

    fn index(&self, idx: [f32; 2]) -> &Self::Output {
        let norm_x = (idx[0] * self.ratio) as usize;
        let norm_y = (idx[1] * self.ratio) as usize;

        &self.cells[(norm_x * self.cols as usize) + norm_y]
    }
}

impl<T> IndexMut<[f32; 2]> for GranularGrid<T> {
    fn index_mut(&mut self, idx: [f32; 2]) -> &mut Self::Output {
        let norm_x = (idx[0] * self.ratio) as usize;
        let norm_y = (idx[1] * self.ratio) as usize;
        &mut self.cells[(norm_x * self.cols as usize) + norm_y]
    }
}