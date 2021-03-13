use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use fast_paths::FastGraph;
use flatbuffers::Vector;
use rayon::prelude::*;

use outbreak_sim::{agents, Bounds, get_root_as_model, read_buffer, TransitGraph, Vec2};
use outbreak_sim::routing::{GranularGrid, nodes_to_granular_grid, sample_nearby_from_grid};

struct InputData<'a> {
    bounds: Bounds,
    transit_graph: TransitGraph<'a>,
    agent_positions: Vec<Vec2>,
    agents_to_workplaces: Vector<'a, u32>,
    workplace_positions: Vec<Vec2>,
    transit_node_grid: GranularGrid<usize>,
}

fn get_input_data(bytes: &Vec<u8>) -> InputData {
    let model = get_root_as_model(bytes);
    let bounds = model.bounds().to_owned();
    let transit_graph = model.transit_graph().to_owned();
    let agents = agents::Agents::new(model.agents().household_index(), model.households().pos());
    let agents_to_workplaces = model.agents().workplace_index().unwrap().to_owned();
    let workplace_positions = model.workplaces().pos().to_owned();
    let transit_node_grid = nodes_to_granular_grid(&transit_graph, &bounds, 200);

    return InputData {
        bounds,
        transit_graph,
        agent_positions: agents.positions,
        agents_to_workplaces,
        workplace_positions,
        transit_node_grid,
    };
}

#[inline]
fn choose_nearby_home_transit_node_sequential(agent_positions: &Vec<Vec2>, workplace_indices: &[u32], transit_node_grid: &GranularGrid<usize>) {
    let mut rng = rand::thread_rng();
    agent_positions.iter().zip(workplace_indices.iter()).for_each(|(pos, workplace_idx)| {
        if *workplace_idx != u32::MAX {
            sample_nearby_from_grid(transit_node_grid, (pos.y(), pos.x()), 8_000.0, &mut rng).unwrap();
        }
    });
}

#[inline]
fn choose_nearby_home_transit_node_parallel(agent_positions: &Vec<Vec2>, workplace_indices: &[u32], transit_node_grid: &GranularGrid<usize>) {
    agent_positions.par_iter().zip(workplace_indices.par_iter())
        .for_each_init(
            || rand::thread_rng(),
            |mut rng, (pos, workplace_idx)| {
                if *workplace_idx != u32::MAX {
                    sample_nearby_from_grid(transit_node_grid, (pos.y(), pos.x()), 8_000.0, &mut rng).unwrap();
                }
            });
}

#[inline]
fn choose_and_calc_workplace_commute(agent_positions: &Vec<Vec2>, workplace_positions: &Vec<Vec2>,
                                     workplace_indices: &[u32], transit_node_grid: &GranularGrid<usize>, fast_graph: &FastGraph) {
    agent_positions.par_iter().zip(workplace_indices.par_iter())
        .for_each_init(
            || (rand::thread_rng(), fast_paths::create_calculator(fast_graph)),
            |(mut rng, path_calculator), (pos, workplace_idx)| {
                if *workplace_idx != u32::MAX {
                    let src_node = sample_nearby_from_grid(transit_node_grid, (pos.y(), pos.x()), 8_000.0, &mut rng).unwrap();
                    let workplace_position = workplace_positions[*workplace_idx as usize];
                    let dest_node = sample_nearby_from_grid(transit_node_grid, (workplace_position.y(), workplace_position.x()), 8_000.0, &mut rng).unwrap();

                    path_calculator.calc_path(&fast_graph, *src_node, *dest_node);
                }
            });
}

fn bench_build_granular_grid(c: &mut Criterion) {
    let mut group = c.benchmark_group("Granular Grid");

    for &model_name in ["model_tower_hamlets", "model_greater_manchester"].iter() {
        for rows in [50u32, 100u32, 200u32].iter() {
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

fn bench_choose_nearby_nodes(c: &mut Criterion) {
    let mut group = c.benchmark_group("Choose Nearby Nodes");

    for &model_name in ["model_tower_hamlets", "model_greater_manchester"].iter() {
        let bytes = read_buffer(&*("python/synthetic_population/output/".to_string() + model_name + ".txt"));
        let input = get_input_data(&bytes);
        group.bench_function(
            BenchmarkId::new("sequential", model_name),
            |b| b.iter(|| choose_nearby_home_transit_node_sequential(
                &input.agent_positions,
                input.agents_to_workplaces.safe_slice(),
                &input.transit_node_grid)
            ),
        );
        group.bench_function(
            BenchmarkId::new("parallel", model_name),
            |b| b.iter(|| choose_nearby_home_transit_node_parallel(
                &input.agent_positions,
                input.agents_to_workplaces.safe_slice(),
                &input.transit_node_grid)
            ),
        );
    }
    group.finish();
}

// TODO Convert this from a batch test to benchmark an individual routing scenario
fn bench_route_commutes(c: &mut Criterion) {
    let mut group = c.benchmark_group("Commute Routing");

    for &model_name in ["model_tower_hamlets", "model_greater_manchester"].iter() {
        let bytes = read_buffer(&*("python/synthetic_population/output/".to_string() + model_name + ".txt"));
        let input = get_input_data(&bytes);
        let fast_graph = fast_paths::load_from_disk(&*("fast_paths/".to_string() + model_name + ".fp")).unwrap();
        group.bench_function(
            BenchmarkId::new("Commute Routing", model_name),
            |b| b.iter(|| choose_and_calc_workplace_commute(
                &input.agent_positions,
                &input.workplace_positions,
                input.agents_to_workplaces.safe_slice(),
                &input.transit_node_grid,
                &fast_graph)
            ),
        );
    }
}

criterion_group!(benches, bench_build_granular_grid, bench_choose_nearby_nodes, bench_route_commutes);
criterion_main!(benches);