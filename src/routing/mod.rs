use fast_paths::{InputGraph, FastGraph};
use crate::flatbuffer::TransitGraph;
use crate::routing::granular_grid::GranularGrid;
use crate::Bounds;
use crate::flatbuffer::Vec2;
use rand::seq::{SliceRandom, IteratorRandom};
use rand::Rng;
use std::cmp::{max, min};

mod granular_grid;

pub fn preprocess_graph(transit_graph: &TransitGraph) -> FastGraph {
    let mut input_graph = InputGraph::new();

    for edge in transit_graph.edges().expect("Transit Graph doesn't have any edges").iter() {
        input_graph.add_edge(edge.start_node_index() as usize, edge.end_node_index() as usize, edge.weight() as usize);
    }

    input_graph.freeze();

    fast_paths::prepare(&input_graph)
}

pub fn nodes_to_granular_grid(transit_graph: &TransitGraph, bounds: &Bounds, rows: u32) -> GranularGrid<usize> {
    let mut grid = GranularGrid::<usize>::new(rows, bounds);
    for (index, node) in transit_graph.nodes().unwrap().iter().enumerate() {
        grid[[node.pos().x(), node.pos().y()]].push(index)
    }

    grid
}

fn get_coords_on_perimeter(center_col: isize, center_row: isize, dist: isize, rows: u32, cols: u32) -> Vec<(u32, u32)> {
    let mut coords = Vec::with_capacity((8 * dist) as usize);

    if center_row >= dist { // Top line
        for col in (max((center_col - dist), 0))..(min((center_col + dist + 1), cols as isize)) { // left to right
            coords.push((col as u32, (center_row - dist) as u32));
        }
    }
    if center_col + dist < cols as isize { // Right Line
        for row in (max((center_row - dist + 1), 0))..(min((center_row + dist), rows as isize)) { // up to down
            coords.push(((center_col + dist) as u32, row as u32));
        }
    }
    if center_row + dist < rows as isize { // Bottom Line
        for col in (max((center_col - dist + 1), 0))..(min((center_col + dist + 1), cols as isize)) { // left to right
            coords.push((col as u32, (center_row + dist) as u32));
        }
    }
    if center_col >= dist { // Left Line
        for row in (max((center_row - dist + 1), 0))..(min((center_row + dist + 1), rows as isize)) { // up to down
            coords.push(((center_col - dist) as u32, row as u32));
        }
    }
    coords
}

pub fn sample_nearby_from_grid<'a, R>(grid: &'a GranularGrid<usize>, centre: (f32, f32), cut_off: f32, rng: &mut R) -> Option<&'a usize>
    where R: Rng + ?Sized {
    let mut dist: u32 = 0;
    let pos = ((centre.0 * grid.ratio) as u32, (centre.1 * grid.ratio) as u32);

    if let Some(chosen) = grid.get_int_index(pos.0, pos.1).choose(rng) {
        return Some(chosen);
    }

    while (dist as f32 / grid.ratio) <= cut_off {
        dist += 1;
        if let Some(chosen) = get_coords_on_perimeter(pos.0 as isize, pos.1 as isize,
                                                      dist as isize, grid.rows, grid.cols).iter()
            .flat_map(|pos| {
                grid.get_int_index(pos.0, pos.1)
            }).choose(rng) {
            return Some(chosen);
        }
    }

    None
}