use std::path::Path;

use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};

use outbreak_sim::{agents, Model, read_buffer};

fn bench_infection_loop(c: &mut Criterion) {
    let mut group = c.benchmark_group("Containers Infection Loop");

    for &model_name in ["tower_hamlets", "greater_manchester", "london_se_commuter_ring"].iter() {
        let mut sim = outbreak_sim::Sim::new(&Path::new("python/synthetic_environments/output"), model_name, true);

        group.bench_function(
            BenchmarkId::new("sequential", model_name),
            |b| b.iter(|| {
                sim.containers.update(&mut agents);
            }),
        );
    }
}

criterion_group!(benches, bench_infection_loop);
criterion_main!(benches);