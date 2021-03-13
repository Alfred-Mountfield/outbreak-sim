use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use flatbuffers::Vector;
use rayon::prelude::*;
use rand::seq::SliceRandom;

use outbreak_sim::{agents, Bounds, get_root_as_model, read_buffer, TransitGraph, Vec2};
use outbreak_sim::routing::{GranularGrid, nodes_to_granular_grid, sample_nearby_from_grid};
use fast_paths::FastGraph;

struct InputData<'a> {
    bounds: Bounds,
    transit_graph: TransitGraph<'a>,
    agent_positions: Vec<Vec2>,
    agents_to_workplaces: Vector<'a, u32>,
    transit_node_grid: GranularGrid<usize>,
}

fn get_input_data(bytes: &Vec<u8>) -> InputData {
    let model = get_root_as_model(bytes);
    let bounds = model.bounds().to_owned();
    let transit_graph = model.transit_graph().to_owned();
    let agents = agents::Agents::new(model.agents().household_index(), model.households().pos());
    let agents_to_workplaces = model.agents().workplace_index().unwrap().to_owned();
    let transit_node_grid = nodes_to_granular_grid(&transit_graph, &bounds, 200);

    return InputData {
        bounds,
        transit_graph,
        agent_positions: agents.positions,
        agents_to_workplaces,
        transit_node_grid,
    };
}

fn bench_build_granular_grid(c: &mut Criterion) {
    let mut group = c.benchmark_group("Granular Grid");

    for &model_name in ["model_tower_hamlets", "model_greater_manchester"].iter() {
        for rows in [50u32, 100u32, 200u32, 400u32].iter() {
            let bytes = read_buffer(&*("python/synthetic_population/output/".to_string() + model_name + ".txt"));
            let input = get_input_data(&bytes);
            group.bench_with_input(
                BenchmarkId::new(model_name, rows), rows,
                |b, rows| b.iter(|| nodes_to_granular_grid(&input.transit_graph, &input.bounds, *rows)),
            );
        }
    }
    group.finish();
}


criterion_group!(benches, bench_build_granular_grid);
criterion_main!(benches);