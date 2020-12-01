// Heavily inspired by https://github.com/parasyte/pixels/blob/master/examples/conway/src/main.rs

use crate::Coord;

#[derive(Clone, Copy, Debug, Default)]
struct Cell {
    pub num_people: u8,
}


impl Cell {
    fn new(num_people: u8) -> Self {
        Self { num_people }
    }


    // #[must_use]
    // fn next_state(mut self, alive: bool) -> Self {
    //     self.alive = alive;
    //     if self.alive {
    //         self.heat = 255;
    //     } else {
    //         self.heat = self.heat.saturating_sub(1);
    //     }
    //     self
    // }
}


pub struct WorldGrid {
    cells: Vec<Cell>,
    width: usize,
    height: usize,
    // Should always be the same size as `cells`. When updating, we read from
    // `cells` and write to `scratch_cells`, then swap. Otherwise it's not in
    // use, and `cells` should be updated directly.
    scratch_cells: Vec<Cell>,
}


impl WorldGrid {
    pub fn new_empty(width: usize, height: usize) -> Self {
        assert!(width != 0 && height != 0);
        let size = width.checked_mul(height).expect("too big");
        Self {
            cells: vec![Cell::default(); size],
            scratch_cells: vec![Cell::default(); size],
            width,
            height,
        }
    }

    pub fn update(&mut self, people: &Vec<Coord>) {
        for y in 0..self.height {
            for x in 0..self.width {
                // let neibs = self.count_neibs(x, y);
                let idx = x + y * self.width;
                // let next = self.cells[idx].update_neibs(neibs);
                // Write into scratch_cells, since we're still reading from `self.cells`
                self.scratch_cells[idx].num_people = 0;
            }
        }
        for person in people.iter() {
            let x = (person.x * self.width as f32) as usize;
            let y = (person.y * self.height as f32) as usize;
            let idx = x + y * self.width;

            let (value, overflow ) = self.scratch_cells[idx].num_people.overflowing_add(50);
            if overflow {
                self.scratch_cells[idx].num_people = u8::MAX;
            } else {
                self.scratch_cells[idx].num_people = value;
            }
        }
        std::mem::swap(&mut self.scratch_cells, &mut self.cells);
    }

    pub fn draw(&self, screen: &mut [u8]) {
        debug_assert_eq!(screen.len(), 4 * self.cells.len());
        for (c, pix) in self.cells.iter().zip(screen.chunks_exact_mut(4)) {
            let color = [0, c.num_people, 0, 0];
            pix.copy_from_slice(&color);
        }
    }

    fn grid_idx<I: std::convert::TryInto<usize>>(&self, x: I, y: I) -> Option<usize> {
        if let (Ok(x), Ok(y)) = (x.try_into(), y.try_into()) {
            if x < self.width && y < self.height {
                Some(x + y * self.width)
            } else {
                None
            }
        } else {
            None
        }
    }
}