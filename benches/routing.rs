use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main, black_box, Throughput};
use fast_paths::FastGraph;
use flatbuffers::Vector;
use rayon::prelude::*;

use outbreak_sim::{agents, get_root_as_model, Model, read_buffer, Vec2};
use outbreak_sim::agents::Agents;
use outbreak_sim::disease::{Uniform, MixingStrategy};
use outbreak_sim::pois::Containers;
use outbreak_sim::routing::{GranularGrid, nodes_to_granular_grid, sample_nearby_from_grid};
use outbreak_sim::Sim;
use rand::{thread_rng, Rng};
use rand::distributions::Standard;

struct InputData<'a> {
    model: Model<'a>,
    agents_pos: Vec<Vec2>,
    containers: Containers<Uniform>,
    transit_node_grid: GranularGrid<usize>,
}

// TODO Update to use the Sim object
fn get_input_data(bytes: &Vec<u8>, mixing_strategy: Uniform) -> InputData {
    let model = get_root_as_model(bytes);
    let containers = Containers::<Uniform>::new(model.households().pos(), model.workplaces().pos(), mixing_strategy);
    let agents_pos = model.agents().household_index().iter().filter_map(|idx| {
        model.households().pos().get(idx as usize)
    }).copied().collect();

    let transit_node_grid = nodes_to_granular_grid(&model.transit_graph(), &model.bounds(), 200);

    return InputData {
        model,
        agents_pos,
        containers,
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
fn choose_and_calc_workplace_transit_commute(agent_positions: &Vec<Vec2>, workplace_positions: &Vec<Vec2>,
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

#[inline]
fn distance_f32(p1: (f32, f32), p2: (f32, f32)) -> f32 {
    black_box(((p2.0 - p1.0).powi(2) + (p2.1 - p1.1).powi(2)).sqrt())
}

#[inline]
fn calc_workplace_direct_commute_1<M: MixingStrategy>(sim: &Sim<M>) {
    sim.agents.occupational_container.iter()
        .enumerate()
        .filter(|(_, work_container_idx)| work_container_idx.is_some())
        .map(|(agent_idx, work_container_idx)| {
            (
                sim.containers.get(sim.agents.household_container[agent_idx]).unwrap().pos,
                sim.containers.get(u64::from(work_container_idx.unwrap())).unwrap().pos
            )
        })
        .map(|(house_container_pos, work_container_pos)| {
            let house_pos = (house_container_pos.x(), house_container_pos.y());
            let work_pos = (work_container_pos.x(), work_container_pos.y());
            (house_pos, work_pos)
        })
        .for_each(|(house_pos, work_pos)| { distance_f32(house_pos, work_pos); });
}


#[inline]
fn calc_workplace_direct_commute_2<M: MixingStrategy>(sim: &Sim<M>) {
    sim.agents.household_container.iter().zip(sim.agents.occupational_container.iter())
        .filter(|(house_container_idx, work_container_idx)| work_container_idx.is_some())
        .map(|(agent_idx, work_container_idx)| {
            (
                sim.containers.get(sim.agents.household_container[*agent_idx as usize]).unwrap().pos,
                sim.containers.get(u64::from(work_container_idx.unwrap())).unwrap().pos
            )
        })
        .map(|(house_container_pos, work_container_pos)| {
            let house_pos = (house_container_pos.x(), house_container_pos.y());
            let work_pos = (work_container_pos.x(), work_container_pos.y());
            (house_pos, work_pos)
        })
        .for_each(|(house_pos, work_pos)| {
            black_box::<(f32, f32)>(house_pos);
        });
}

fn bench_build_granular_grid(c: &mut Criterion) {
    let mut group = c.benchmark_group("Granular Grid");

    for &model_name in ["model_tower_hamlets", "model_greater_manchester"].iter() {
        for rows in [50u32, 100u32, 200u32].iter() {
            let bytes = read_buffer(&*("python/synthetic_population/output/".to_string() + model_name + ".txt"));
            let mixing_strategy = Uniform { transmission_chance: 0.02 };
            let input = get_input_data(&bytes, mixing_strategy);
            group.bench_with_input(
                BenchmarkId::new(model_name, rows), rows,
                |b, rows| b.iter(|| nodes_to_granular_grid(&input.model.transit_graph(), &input.model.bounds(), *rows)),
            );
        }
    }
    group.finish();
}

fn bench_choose_nearby_nodes(c: &mut Criterion) {
    let mut group = c.benchmark_group("Choose Nearby Nodes");

    for &model_name in ["model_tower_hamlets", "model_greater_manchester"].iter() {
        let bytes = read_buffer(&*("python/synthetic_population/output/".to_string() + model_name + ".txt"));
        let mixing_strategy = Uniform { transmission_chance: 0.02 };
        let input = get_input_data(&bytes, mixing_strategy);
        group.bench_function(
            BenchmarkId::new("sequential", model_name),
            |b| b.iter(|| choose_nearby_home_transit_node_sequential(
                &input.agents_pos,
                input.model.agents().workplace_index().safe_slice(),
                &input.transit_node_grid)
            ),
        );
        group.bench_function(
            BenchmarkId::new("parallel", model_name),
            |b| b.iter(|| choose_nearby_home_transit_node_parallel(
                &input.agents_pos,
                input.model.agents().workplace_index().safe_slice(),
                &input.transit_node_grid)
            ),
        );
    }
    group.finish();
}

// TODO Convert this from a batch test to benchmark an individual routing scenario
fn bench_route_transit_commutes(c: &mut Criterion) {
    let mut group = c.benchmark_group("Commute Routing by Transit");

    for &model_name in ["model_tower_hamlets", "model_greater_manchester"].iter() {
        let bytes = read_buffer(&*("python/synthetic_population/output/".to_string() + model_name + ".txt"));
        let mixing_strategy = Uniform { transmission_chance: 0.02 };
        let input = get_input_data(&bytes, mixing_strategy);
        let fast_graph = fast_paths::load_from_disk(&*("fast_paths/".to_string() + model_name + ".fp")).unwrap();

        group.bench_function(
            BenchmarkId::new("Commute Routing", model_name),
            |b| b.iter(|| choose_and_calc_workplace_transit_commute(
                &input.agents_pos,
                &input.model.workplaces().pos().to_owned(),
                input.model.agents().workplace_index().safe_slice(),
                &input.transit_node_grid,
                &fast_graph)
            ),
        );
    }
    group.finish();
}

fn bench_direct_commute_calc(c: &mut Criterion) {
    let mut group = c.benchmark_group("Direct Commute Routing (non-transit)");

    for &model_name in ["model_tower_hamlets", "model_greater_manchester"].iter() {
        let mut sim = Sim::new(model_name, true);

        group.bench_function(
            BenchmarkId::new("Commute Routing Filter Index", model_name),
            |b| b.iter(|| calc_workplace_direct_commute_1(&sim)
            ),
        );
        group.bench_function(
            BenchmarkId::new("Commute Routing Zip Filter", model_name),
            |b| b.iter(|| calc_workplace_direct_commute_2(&sim)
            ),
        );
    }
    group.finish();
}

fn bench_distance(c: &mut Criterion) {
    let mut group = c.benchmark_group("euc dists");

    for num in [100, 1_000, 10_000, 1_000_000, 10_000_000].iter() {
        let points: Vec<(f32, f32)> = thread_rng().sample_iter(Standard)
            .zip(thread_rng().sample_iter(Standard))
            .take(num * 2).collect();

        let (left, right) = points.as_slice().split_at(points.len() / 2);
        group.throughput(Throughput::Elements(*num as u64));
        group.bench_with_input(BenchmarkId::from_parameter(num), num, |b, _| {
            b.iter(|| {
                left.iter().zip(right.iter())
                    .for_each(|(left, right)| { distance_f32(*left, *right); });
            });
        });
    }
    group.finish();
}

criterion_group!(benches, bench_build_granular_grid, bench_choose_nearby_nodes, bench_route_transit_commutes, bench_direct_commute_calc, bench_distance);
criterion_main!(benches);