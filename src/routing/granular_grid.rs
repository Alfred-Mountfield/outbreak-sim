use crate::flatbuffer::TransitNode;
use std::iter;
use std::ops::{Index, IndexMut};
use std::ops;

pub struct GranularGrid<T> {
    rows: u32,
    cols: u32,
    cells: Vec<Vec<T>>
}

impl<T> GranularGrid<T> {
    pub fn new(rows: u32, cols: u32) -> Self {
        let size = (rows as usize).checked_mul(cols as usize).expect("too big");
        let cells = iter::repeat_with(|| Vec::<T>::new()).take(size).collect();
        Self {
            rows,
            cols,
            cells
        }
    }
}

impl<T> Index<[usize; 2]> for GranularGrid<T> {
    type Output = Vec<T>;

    fn index(&self, idx: [usize; 2]) -> &Self::Output {
        &self.cells[idx[0] * self.cols as usize + idx[1]]
    }
}

impl<T> IndexMut<[usize; 2]> for GranularGrid<T> {
    fn index_mut(&mut self, idx: [usize; 2]) -> &mut Self::Output {
        &mut self.cells[idx[0] * self.cols as usize + idx[1]]
    }
}