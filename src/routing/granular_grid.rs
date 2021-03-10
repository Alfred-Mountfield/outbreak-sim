use crate::flatbuffer::TransitNode;
use std::iter;
use std::ops::{Index, IndexMut};
use std::ops;
use crate::Bounds;

#[derive(PartialEq, Debug)]
pub struct GranularGrid<T> {
    pub rows: u32,
    pub cols: u32,
    pub idx_to_coord_ratio: f32,
    pub bounds: Bounds,
    cells: Vec<Vec<T>>,
}

/// # Granular Grid
/// A Spatial Enumeration structure, bucketing a given dataset into a grid with a given granularity.
/// The underlying space is defined by a set of Bounds, where each cell is expected to be square in
/// terms of the underlying coordinate system.
///
/// The Grid itself uses a single vec for its linear storage in row-major order. It can be indexed
/// directly through the cell's (row,column) indices or by the (y,x) coordinates of the coordinate
/// system. Each cell contains a vec of the respective pieces of data for that square of space.
impl<T> GranularGrid<T> {
    /// Returns a new GranularGrid ready for data-insertion
    ///
    /// # Arguments
    ///
    /// * `rows` - Number of rows to use to partition the space, this defines the ratio (granularity)
    /// of the grid by rows / bounds.max().x()
    ///
    /// * `bounds` - Bounding box of the underlying space (non-inclusive). Number of columns is
    /// decided by the calculated ratio and the maximum y value
    pub fn new(rows: u32, bounds: &Bounds) -> Self {
        let idx_to_coord_ratio = rows as f32 / bounds.max().y() as f32;
        let cols = (bounds.max().x() * idx_to_coord_ratio).ceil() as u32;
        let size = (rows as usize).checked_mul(cols as usize).expect("too big");

        let cells = iter::repeat_with(|| Vec::<T>::new()).take(size).collect();
        Self {
            rows,
            cols,
            idx_to_coord_ratio,
            bounds: bounds.to_owned(),
            cells,
        }
    }

    /// Returns the vector of elements at a given cell's (row,col) index
    pub fn get_int_index(&self, row: u32, col: u32) -> &Vec<T> {
        &self.cells[row as usize * self.cols as usize + col as usize]
    }
}

impl<T> Index<[f32; 2]> for GranularGrid<T> {
    type Output = Vec<T>;

    /// Given a `[y,x]` coordinate, it returns a reference to the contents of the cell that
    /// contains that coordinate
    fn index(&self, coord: [f32; 2]) -> &Self::Output {
        let row_idx = (coord[0] * self.idx_to_coord_ratio) as usize;
        let col_idx = (coord[1] * self.idx_to_coord_ratio) as usize;

        &self.cells[(row_idx * self.cols as usize) + col_idx]
    }
}

impl<T> IndexMut<[f32; 2]> for GranularGrid<T> {
    /// Given a `[y,x]` coordinate, it returns a mutable reference to the contents of the cell that
    /// contains that coordinate
    fn index_mut(&mut self, coord: [f32; 2]) -> &mut Self::Output {
        let row_idx = (coord[0] * self.idx_to_coord_ratio) as usize;
        let col_idx = (coord[1] * self.idx_to_coord_ratio) as usize;
        &mut self.cells[(row_idx * self.cols as usize) + col_idx]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::flatbuffer::Vec2;

    #[test]
    fn test_new_granular_grid() {
        let rows = 10;
        let expected_cols = 8;
        let expected_ratio = 1.0 / 10.0;
        let expected_len = (rows * expected_cols) as usize;

        let bounds = Bounds::new(&Vec2::new(0.0, 0.0), &Vec2::new(80.0, 100.0));
        let grid = GranularGrid::<u32>::new(10, &bounds);

        assert_eq!(grid.rows, rows);
        assert_eq!(grid.cols, expected_cols);
        assert_eq!(grid.idx_to_coord_ratio, expected_ratio);
        assert_eq!(grid.cells.len(), expected_len);
    }

    #[test]
    fn test_granular_index() {
        let bounds = Bounds::new(&Vec2::new(0.0, 0.0), &Vec2::new(130.0, 100.0));
        let mut grid = GranularGrid::<i32>::new(2, &bounds);

        println!("{}", grid.cols);

        grid[[0.1, 55.0]].push(1);
        grid[[53.0, 0.1]].push(2);
        grid[[90.0, 124.0]].push(3);
        let expected = vec![
            vec![], vec![1], vec![],
            vec![2], vec![], vec![3]
        ];

        assert_eq!(grid.cells, expected);
        assert_eq!(grid.get_int_index(1, 0)[0], 2);
    }
}