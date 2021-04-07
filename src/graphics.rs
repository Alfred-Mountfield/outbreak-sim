use std::cmp::{min, max};

// Heavily inspired by https://github.com/parasyte/pixels/blob/master/examples/conway/src/main.rs
use outbreak_sim::{disease, Sim};
use outbreak_sim::disease::MixingStrategy;

#[derive(Clone, Copy, Debug, Default)]
struct Cell {
    total: u32,
    num_susceptible: u32,
    num_presymptomatic: u32,
    num_infectious: u32,
    num_recovered: u32
}

pub struct WorldGrid {
    cells: Vec<Cell>,
    width: usize,
    height: usize,
    max_at_cell: u32,
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
            max_at_cell: 0,
            width,
            height,
        }
    }

    pub fn update<M: MixingStrategy>(&mut self, sim: &Sim<M>) {
        self.max_at_cell = 0;
        for y in 0..self.height {
            for x in 0..self.width {
                let idx = x + y * self.width;
                // Write into scratch_cells, since we're still reading from `self.cells`
                self.scratch_cells[idx] = Cell::default();
            }
        }
        for container_idx in 0..sim.containers.len() {
            let container = sim.containers.get(container_idx as u64).unwrap();
            let mut x = ((container.pos.x() / sim.bounds.max().x()) * self.width as f32) as usize;
            let mut y = ((((container.pos.y() / sim.bounds.max().y()) - 1.0) * self.height as f32).abs()) as usize;
            x = min(x, self.width - 1); y = min(y, self.height - 1);
            let idx = x + y * self.width;

            let cell_stats = &mut self.scratch_cells[idx];
            for &agent_idx in container.inhabitants.iter() {
                match sim.agents.disease_statuses[agent_idx as usize].state {
                    disease::State::Susceptible => {
                        cell_stats.num_susceptible += 1;
                    },
                    disease::State::Presymptomatic => {
                        cell_stats.num_presymptomatic += 1;
                    }
                    disease::State::Infectious => {
                        cell_stats.num_infectious += 1;
                    },
                    disease::State::Recovered => {
                        cell_stats.num_recovered += 1;
                    }
                }
            }
            cell_stats.total += container.inhabitants.len() as u32;
            self.max_at_cell = max(cell_stats.total, self.max_at_cell);
        }
        std::mem::swap(&mut self.scratch_cells, &mut self.cells);
    }

    pub fn draw(&self, screen: &mut [u8]) {
        debug_assert_eq!(screen.len(), 4 * self.cells.len());
        for (c, pix) in self.cells.iter().zip(screen.chunks_exact_mut(4)) {

            let pop_density = c.total as f32 / self.max_at_cell as f32;
            let infected_ratio = c.num_infectious as f32 / c.total as f32;

            let color = [(infected_ratio * 255.0) as u8, (pop_density * (1.0 - infected_ratio) * 255.0) as u8, 0, 0];
            pix.copy_from_slice(&color);
        }
    }
}