use std::path::Path;

use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main, Throughput};
use fast_paths::FastGraph;
use nonmax::NonMaxU64;
use rand::{Rng, thread_rng};
use rand::distributions::Standard;
use rayon::prelude::*;

use outbreak_sim::{get_root_as_model, read_buffer, Vec2};
use outbreak_sim::disease::MixingStrategy;
use outbreak_sim::routing::{calculate_direct_commute_time, DirectRoutingType, distance_f32, GranularGrid, nodes_to_granular_grid, sample_nearby_from_grid};
use outbreak_sim::Sim;

#[inline]
fn choose_nearby_home_transit_node_sequential(agent_positions: &[Vec2], transit_node_grid: &GranularGrid<usize>) {
    let mut rng = rand::thread_rng();
    agent_positions.iter().for_each(|pos| {
        sample_nearby_from_grid(transit_node_grid, (pos.y(), pos.x()), 8_000.0, 1, &mut rng).unwrap();
    });
}

#[inline]
fn choose_nearby_home_transit_node_parallel(agent_positions: &[Vec2], transit_node_grid: &GranularGrid<usize>) {
    agent_positions.par_iter()
        .for_each_init(
            rand::thread_rng,
            |mut rng, pos| {
                sample_nearby_from_grid(transit_node_grid, (pos.y(), pos.x()), 8_000.0, 1, &mut rng).unwrap();
            });
}

#[inline]
fn choose_and_calc_workplace_transit_commute(agent_positions: &[Vec2], workplace_positions: &[Vec2],
                                             transit_node_grid: &GranularGrid<usize>, fast_graph: &FastGraph) {
    agent_positions.par_iter().zip(workplace_positions.par_iter())
        .for_each_init(
            || (rand::thread_rng(), fast_paths::create_calculator(fast_graph)),
            |(rng, path_calculator), (household_pos, workplace_pos)| {
                let mut rng = rng;
                let src_node = sample_nearby_from_grid(transit_node_grid, (household_pos.y(), household_pos.x()), 8_000.0, 1, &mut rng).unwrap();
                let dest_node = sample_nearby_from_grid(transit_node_grid, (workplace_pos.y(), workplace_pos.x()), 8_000.0, 1, &mut rng).unwrap();

                path_calculator.calc_path(&fast_graph, src_node[0], dest_node[0]);
            });
}

#[inline]
fn calc_workplace_direct_commute<M: MixingStrategy>(sim: &Sim<M>, household_containers: &[NonMaxU64], occupational_containers: &[NonMaxU64]) {
    household_containers.par_iter().zip(occupational_containers.par_iter())
        .for_each(|(&household_container_idx, &occupational_container_idx)| {
            calculate_direct_commute_time(&sim.containers, DirectRoutingType::Driving,
                                          household_container_idx, occupational_container_idx);
        });
}

fn bench_build_granular_grid(c: &mut Criterion) {
    let mut group = c.benchmark_group("Granular Grid");

    for &model_name in ["tower_hamlets", "greater_manchester"].iter() {
        for rows in [50u32, 100u32, 200u32].iter() {
            let bytes = read_buffer(("python/synthetic_environments/examples/".to_string() + model_name + ".txt").as_ref());
            let model = get_root_as_model(&bytes);
            group.bench_with_input(
                BenchmarkId::new(model_name, rows), rows,
                |b, rows| b.iter(|| nodes_to_granular_grid(&model.transit_graph(), &model.bounds(), *rows)),
            );
        }
    }
    group.finish();
}

fn bench_choose_nearby_nodes(c: &mut Criterion) {
    let mut group = c.benchmark_group("Choose Nearby Nodes");

    for &model_name in ["tower_hamlets", "greater_manchester"].iter() {
        let sim = outbreak_sim::Sim::new(&Path::new("python/synthetic_environments/examples"), model_name, true);
        let agent_positions: Vec<Vec2> = sim.agents.household_container.iter()
            .zip(sim.agents.occupational_container.iter())
            .filter_map(|(&household_idx, occupational_idx)| {
                if occupational_idx.is_some() { Some(sim.containers.get(household_idx).unwrap().pos) } else { None }
            }).collect();

        group.bench_function(
            BenchmarkId::new("sequential", model_name),
            |b| b.iter(|| choose_nearby_home_transit_node_sequential(
                &agent_positions,
                &sim.transit_granular_grid)
            ),
        );
        group.bench_function(
            BenchmarkId::new("parallel", model_name),
            |b| b.iter(|| choose_nearby_home_transit_node_parallel(
                &agent_positions,
                &sim.transit_granular_grid)
            ),
        );
    }
    group.finish();
}

// TODO Convert this from a batch test to benchmark an individual routing scenario
fn bench_route_transit_commutes(c: &mut Criterion) {
    let mut group = c.benchmark_group("Commute Routing by Transit");

    for &model_name in ["tower_hamlets", "greater_manchester"].iter() {
        let sim = outbreak_sim::Sim::new(&Path::new("python/synthetic_environments/examples"), model_name, true);
        let (agent_positions, workplace_positions): (Vec<Vec2>, Vec<Vec2>) = sim.agents.household_container.iter()
            .zip(sim.agents.occupational_container.iter())
            .filter_map(|(&household_idx, occupational_idx)| {
                if let Some(work_idx) = occupational_idx {
                    Some((sim.containers.get(household_idx).unwrap().pos, sim.containers.get(work_idx.get()).unwrap().pos))
                } else {
                    None
                }
            }).unzip();

        group.bench_function(
            BenchmarkId::new("Commute Routing", model_name),
            |b| b.iter(|| choose_and_calc_workplace_transit_commute(
                agent_positions.as_slice(),
                workplace_positions.as_slice(),
                &sim.transit_granular_grid,
                &sim.fast_graph,
            )
            ),
        );
    }
    group.finish();
}

fn bench_direct_commute_calc(c: &mut Criterion) {
    let mut group = c.benchmark_group("Direct Commute Routing (non-transit)");

    for &model_name in ["tower_hamlets", "greater_manchester"].iter() {
        let sim = outbreak_sim::Sim::new(&Path::new("python/synthetic_environments/examples"), model_name, true);
        let (household_containers, occupational_containers): (Vec<NonMaxU64>, Vec<NonMaxU64>) = sim.agents.household_container.iter()
            .zip(sim.agents.occupational_container.iter())
            .filter_map(|(&household_container_idx, &occupational_container_idx)| {
                if let Some(occupational_idx) = occupational_container_idx {
                    Some((NonMaxU64::new(household_container_idx).unwrap(), occupational_idx))
                } else {
                    None
                }
            }).unzip();

        group.bench_function(
            BenchmarkId::new("Commute Direct Routing", model_name),
            |b| b.iter(|| calc_workplace_direct_commute(&sim, &household_containers, &occupational_containers)),
        );
    }
    group.finish();
}

fn bench_distance(c: &mut Criterion) {
    let mut group = c.benchmark_group("euc dists");

    for num in [100, 1_000, 10_000, 1_000_000, 10_000_000].iter() {
        let points: Vec<Vec2> = thread_rng().sample_iter(Standard)
            .zip(thread_rng().sample_iter(Standard))
            .take(num * 2)
            .map(|(x, y)| Vec2::new(x, y))
            .collect();

        let (left, right) = points.as_slice().split_at(points.len() / 2);
        group.throughput(Throughput::Elements(*num as u64));
        group.bench_with_input(BenchmarkId::from_parameter(num), num, |b, _| {
            b.iter(|| {
                left.iter().zip(right.iter())
                    .for_each(|(&left, &right)| { distance_f32(left, right); });
            });
        });
    }
    group.finish();
}

criterion_group!(benches, bench_build_granular_grid, bench_choose_nearby_nodes, bench_route_transit_commutes, bench_direct_commute_calc, bench_distance);
criterion_main!(benches);