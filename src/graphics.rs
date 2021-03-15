// Heavily inspired by https://github.com/parasyte/pixels/blob/master/examples/conway/src/main.rs
use outbreak_sim::{Bounds, disease};
use outbreak_sim::agents::Agents;

#[derive(Clone, Copy, Debug, Default)]
struct Cell {
    // [Susceptible, Infectious, Recovered]
    pub num_people_with_ds: [u8; 3],
}


impl Cell {
    fn new() -> Self {
        Self {
            num_people_with_ds: [0; 3]
        }
    }
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

    pub fn update(&mut self, agents: &Agents, bounds: &Bounds) {
        for y in 0..self.height {
            for x in 0..self.width {
                let idx = x + y * self.width;
                // Write into scratch_cells, since we're still reading from `self.cells`
                self.scratch_cells[idx].num_people_with_ds = [0, 0, 0];
            }
        }
        for i in 0..agents.num_agents {
            // let x = ((agents.positions[i as usize].x() / bounds.max().x()) * self.width as f32) as usize;
            // let y = ((((agents.positions[i as usize].y() / bounds.max().y()) - 1.0) * self.height as f32).abs()) as usize;
            // let idx = x + y * self.width;
            //
            // let num_people = &mut self.scratch_cells[idx].num_people_with_ds;
            // match agents.disease_statuses[i as usize].state {
            //     disease::State::Susceptible => {
            //         num_people[0] = num_people[0].saturating_add(150);
            //     },
            //     disease::State::Infectious => {
            //         num_people[1] = num_people[1].saturating_add(150);
            //     },
            //     disease::State::Recovered => {
            //         num_people[1] = num_people[1].saturating_add(150);
            //     }
            // }
        }
        std::mem::swap(&mut self.scratch_cells, &mut self.cells);
    }

    pub fn draw(&self, screen: &mut [u8]) {
        // debug_assert_eq!(screen.len(), 4 * self.cells.len());
        for (c, pix) in self.cells.iter().zip(screen.chunks_exact_mut(4)) {
            // println!("{},{},{}", c.num_people_with_ds[0], c.num_people_with_ds[1], c.num_people_with_ds[2]);
            let color = [c.num_people_with_ds[1], c.num_people_with_ds[0], c.num_people_with_ds[2], 0];
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