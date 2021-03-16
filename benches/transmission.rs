use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use rayon::prelude::*;

use outbreak_sim::{agents, get_root_as_model, Model, read_buffer};
use outbreak_sim::disease::Uniform;
use outbreak_sim::pois::Containers;

struct InputData<'a> {
    model: Model<'a>,
    containers: Containers<'a, Uniform>,
}

fn get_input_data<'a>(bytes: &'a Vec<u8>, mixing_strategy: &'a Uniform) -> InputData<'a> {
    let model = get_root_as_model(bytes);
    let containers = Containers::<Uniform>::new(model.households().pos(), model.workplaces().pos(), &mixing_strategy);

    return InputData {
        model,
        containers,
    };
}

fn bench_infection_loop(c: &mut Criterion) {
    let mut group = c.benchmark_group("Containers Infection Loop");

    for &model_name in ["model_tower_hamlets", "model_greater_manchester", "model_london_se_commuter_ring"].iter() {
        let bytes = read_buffer(&*("python/synthetic_population/output/".to_string() + model_name + ".txt"));
        let mixing_strategy = Uniform { transmission_chance: 0.02 };
        let mut input = get_input_data(&bytes, &mixing_strategy);
        let mut agents = agents::Agents::new(&input.model, &mut input.containers);

        group.bench_function(
            BenchmarkId::new("sequential", model_name),
            |b| b.iter(|| {
                input.containers.update(&mut agents);
            }),
        );
    }
}

criterion_group!(benches, bench_infection_loop);
criterion_main!(benches);