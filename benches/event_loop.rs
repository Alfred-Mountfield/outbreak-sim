use std::path::Path;

use criterion::{BatchSize, BenchmarkId, Criterion, criterion_group, criterion_main, Throughput};
use outbreak_sim::shared::get_time_steps_per_day;

// TODO When global param modification is added, make sure setup is done in bench for consistency
fn bench_event_loop(c: &mut Criterion) {
    let mut group = c.benchmark_group("Event Loop");
    for (model_dir, model_name) in [("python/synthetic_environments/examples", "isle_of_dogs"),
        ("python/synthetic_environments/examples", "greater_manchester"), ("python/synthetic_environments/output", "london_se_commuter_ring")].iter()
    {
        let sim = outbreak_sim::SimBuilder::new(&Path::new(model_dir), model_name)
            .load_fast_graph_from_disk(true)
            .build();
        let num_agents = sim.agents.num_agents as u64;
        group.throughput(Throughput::Elements(num_agents));
        group.bench_with_input(BenchmarkId::new(format!("One Day: {} time-steps", get_time_steps_per_day()), model_name), &num_agents, |b, _| {
            b.iter_batched(
                // TODO change this to .clone() when FastGraphs updates with derive
                || {
                    let sim = outbreak_sim::SimBuilder::new(&Path::new(model_dir), model_name)
                        .load_fast_graph_from_disk(true)
                        .build();
                    let path_calculator = fast_paths::create_calculator(&sim.fast_graph);
                    (sim, path_calculator)
                },
                |(mut sim, mut path_calculator)| {
                    for time_step in 0..get_time_steps_per_day() {
                        sim.events.update(time_step, &mut sim.agents, &mut sim.containers,
                                          &sim.transit_granular_grid, &sim.fast_graph,
                                          &mut path_calculator)
                    }
                },
                BatchSize::LargeInput);
        });
    }
    group.finish();
}

criterion_group! {
    name = benches;
    config = Criterion::default().sample_size(10);
    targets = bench_event_loop
}
criterion_main!(benches);