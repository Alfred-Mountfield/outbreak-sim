use std::cmp::{max, min};

use fast_paths::{FastGraph, PathCalculator};
use nonmax::NonMaxU64;
use rand::{Rng, thread_rng};
use rand::seq::{IteratorRandom, SliceRandom};

use crate::{Bounds, Vec2};
use crate::containers::Containers;
use crate::disease::MixingStrategy;
use crate::flatbuffer::TransitGraph;
pub use crate::routing::granular_grid::GranularGrid;
use crate::shared::{get_cycling_speed, get_driving_speed, get_walking_speed};
use crate::shared::types::TimeStep;

pub mod transit;
mod granular_grid;

#[derive(Debug, Copy, Clone)]
pub enum RoutingType {
    Transit,
    Direct(DirectRoutingType),
}

#[derive(Debug, Copy, Clone)]
pub enum DirectRoutingType {
    Walking,
    Cycling,
    Driving,
}

#[inline]
pub fn distance_f32(p1: Vec2, p2: Vec2) -> f32 {
    ((p2.x() - p1.x()).powi(2) + (p2.y() - p1.y()).powi(2)).sqrt()
}

#[inline]
pub fn calculate_direct_commute_time<M>(containers: &Containers<M>, routing_type: DirectRoutingType,
                                        from_container_idx: NonMaxU64, to_container_idx: NonMaxU64) -> TimeStep
    where M: MixingStrategy
{
    let p1 = containers.get(from_container_idx.get()).unwrap().pos;
    let p2 = containers.get(to_container_idx.get()).unwrap().pos;
    let dist = distance_f32(p1, p2);

    (dist / match routing_type {
        DirectRoutingType::Walking => {
            get_walking_speed()
        }
        DirectRoutingType::Cycling => {
            get_cycling_speed()
        }
        DirectRoutingType::Driving => {
            get_driving_speed()
        }
    }) as TimeStep
}

#[inline]
pub fn calculate_public_transit_commute_time<'e, M>(containers: &Containers<M>, transit_grid: &GranularGrid<usize>,
                                                    transit_path_calculator: &mut PathCalculator, fast_graph: &FastGraph,
                                                    from_container_idx: NonMaxU64, to_container_idx: NonMaxU64) -> Result<TimeStep, &'e str>
    where M: MixingStrategy
{
    let start_pos = containers.get(from_container_idx.get()).unwrap().pos;
    let end_pos = containers.get(to_container_idx.get()).unwrap().pos;
    let mut rng = thread_rng();

    let possible_start_nodes = sample_nearby_from_grid(transit_grid, (start_pos.y(), start_pos.x()), 3_500.0, 5, &mut rng);
    let possible_end_nodes = sample_nearby_from_grid(transit_grid, (end_pos.y(), end_pos.x()), 3_500.0, 5, &mut rng);

    if let (Some(start_nodes), Some(end_nodes)) = (possible_start_nodes, possible_end_nodes) {
        for end_node in end_nodes {
            for start_node in &start_nodes {
                if let Some(shortest_path) = transit_path_calculator.calc_path(fast_graph, *start_node, end_node) {
                    return Ok(shortest_path.get_weight() as TimeStep);
                }
            }
        }
    }
    Err("No suitable paths were found")
}

/// Creates a GranularGrid of TransitNodes
pub fn nodes_to_granular_grid(transit_graph: &TransitGraph, bounds: &Bounds, rows: u32) -> GranularGrid<usize> {
    let mut grid = GranularGrid::<usize>::new(rows, bounds);
    for (index, node) in transit_graph.nodes().iter().enumerate() {
        grid[[node.pos().y(), node.pos().x()]].push(index)
    }

    grid
}

/// Returns (row, col) coordinates of the perimeter of a square on a grid, taking into consideration
/// the bounds of the grid, only returning valid indices.
///
///  # Arguments
/// * `center_row` - The row index of the center point of the square
/// * `center_col` - The column index of the center point of the square
/// * `dist` - The radius of the square, i.e. the perimeter's distance from the center
/// * `rows` - The number of rows in the grid
/// * `cols` - The number of columns in the grid
fn get_coords_on_perimeter(center_row: isize, center_col: isize, dist: isize, rows: u32, cols: u32) -> Vec<(u32, u32)> {
    let mut coords = Vec::with_capacity((8 * dist) as usize);

    if center_row >= dist { // Top line
        for col in (max(center_col - dist, 0))..(min(center_col + dist + 1, cols as isize)) { // left to right
            coords.push(((center_row - dist) as u32, col as u32));
        }
    }
    if center_col + dist < cols as isize { // Right Line
        for row in (max(center_row - dist + 1, 0))..(min(center_row + dist, rows as isize)) { // up to down
            coords.push((row as u32, (center_col + dist) as u32));
        }
    }
    if center_row + dist < rows as isize { // Bottom Line
        for col in (max(center_col - dist + 1, 0))..(min(center_col + dist + 1, cols as isize)) { // left to right
            coords.push(((center_row + dist) as u32, col as u32));
        }
    }
    if center_col >= dist { // Left Line
        for row in (max(center_row - dist + 1, 0))..(min(center_row + dist + 1, rows as isize)) { // up to down
            coords.push((row as u32, (center_col - dist) as u32));
        }
    }
    coords
}

/// Tries to sample an element from a GranularGrid, checking in squares of increasing size from the
/// cell of a given co-ordinate.
///
///  # Arguments
/// * `grid` - The GranularGrid containing the elements to sample from
/// * `centre` - A (y,x) co-ordinate to approximately search around
/// * `cut_off` - The approximate maximum distance at which to stop searching
/// * `rng` - Rng to pass to the choose() function for sampling
#[inline]
pub fn sample_nearby_from_grid<R>(grid: &GranularGrid<usize>, centre: (f32, f32), cut_off: f32, num_samples: usize, rng: &mut R) -> Option<Vec<usize>>
    where R: Rng + ?Sized {
    let mut dist: u32 = 0;
    let pos = ((centre.0 * grid.idx_to_coord_ratio) as u32, (centre.1 * grid.idx_to_coord_ratio) as u32);

    if let Some(chosen) = grid.get_int_index(pos.0, pos.1).choose(rng) {
        return Some(vec![*chosen]);
    }

    while (dist as f32 / grid.idx_to_coord_ratio) <= cut_off {
        dist += 1;
        let sampled = get_coords_on_perimeter(pos.0 as isize, pos.1 as isize, dist as isize, grid.rows, grid.cols)
            .into_iter()
            .flat_map(|pos| {
                grid.get_int_index(pos.0, pos.1).clone()
            })
            .choose_multiple(rng, num_samples);
        if !sampled.is_empty() {
            return Some(sampled);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_coords_on_perimeter() {
        let coords = get_coords_on_perimeter(2, 0, 0, 7, 7);
        let expected = vec![(2, 0)];
        assert_eq!(coords, expected);

        let coords = get_coords_on_perimeter(2, 0, 1, 7, 7);
        let expected = vec![
            (1, 0), (1, 1),
            (2, 1),
            (3, 0), (3, 1)
        ];
        assert_eq!(coords, expected);

        let coords = get_coords_on_perimeter(4, 5, 3, 7, 7);
        let expected = vec![
            (1, 2), (1, 3), (1, 4), (1, 5), (1, 6),
            (2, 2),
            (3, 2),
            (4, 2),
            (5, 2),
            (6, 2),
        ];
        assert_eq!(coords, expected);
    }
}