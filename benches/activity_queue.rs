use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main, Throughput, BatchSize};
use std::collections::{VecDeque};

use rand::{thread_rng, Rng};
use rayon::prelude::*;

const DAYS: u16 = 30;
const TIME_STEPS_PER_DAY: u16 = 48;

#[derive(Debug, Copy, Clone)]
struct Activity {
    agent_idx: u64,
    start_timestep: u16,
    end_timestep: u16,
}

trait VecDequeMutExt<T: Default> {
    fn get_or_grow(&mut self, index: usize) -> Option<&T>;
    fn get_mut_or_grow(&mut self, index: usize) -> Option<&mut T>;
}

impl<T: Default> VecDequeMutExt<T> for VecDeque<T> {
    fn get_or_grow(&mut self, index: usize) -> Option<&T> {
        if index >= self.len() {
            self.resize_with(index + 1, Default::default);
        }
        self.get(index)
    }
    fn get_mut_or_grow(&mut self, index: usize) -> Option<&mut T> {
        if index >= self.len() {
            self.resize_with(index + 1, Default::default);
        }
        self.get_mut(index)
    }
}

fn build_agent_activities(num_agents: u64) -> Vec<Vec<Activity>> {
    let mut agents_to_activities = Vec::with_capacity(num_agents as usize);
    let mut rng = thread_rng();

    for agent_idx in 0..num_agents {
        let mut end_timestep = DAYS * TIME_STEPS_PER_DAY;
        let mut activities = Vec::new();

        while end_timestep > 0 {
            let start_timestep = end_timestep.saturating_sub(rng.gen_range(2..(TIME_STEPS_PER_DAY / 2)));

            activities.push(Activity {
                agent_idx,
                start_timestep,
                end_timestep,
            });

            end_timestep = start_timestep;
        };

        agents_to_activities.push(activities);
    };

    agents_to_activities
}

#[inline]
fn activity_loop(mut activities_at_timestep: VecDeque<Vec<Activity>>, mut agents_to_activities: Vec<Vec<Activity>>) {
    for time_step in 0..(DAYS * TIME_STEPS_PER_DAY) {
        if let Some(mut activities) = activities_at_timestep.pop_front() {
            activities.drain(..).for_each(|activity| {
                if let Some(next_activity) = agents_to_activities[activity.agent_idx as usize].pop() {
                    activities_at_timestep.get_mut_or_grow((next_activity.end_timestep - time_step) as usize).unwrap().push(next_activity);
                }
            });
        }
    };
}

// #[inline]
// fn activity_loop_parallel(activities_at_timestep: DashMap<u16, Vec<Activity>>, mut agents_to_activities: Vec<Vec<Activity>>) {
//     let agents_to_activities = Mutex::new(agents_to_activities);
//     for time_step in 0..(DAYS * TIME_STEPS_PER_DAY) {
//         if let Some(mut events) = activities_at_timestep.remove(&time_step) {
//             events.1.par_drain(..).for_each(|activity| {
//                 if let Some(next_activity) = { let x = agents_to_activities.lock().unwrap(); x }[activity.agent_idx as usize].pop() {
//                     activities_at_timestep.entry(next_activity.end_timestep)
//                         .or_insert(vec![])
//                         .push(next_activity);
//                 }
//             });
//         }
//     };
// }

fn bench_activity_loop(c: &mut Criterion) {
    let agents_to_activities_master = build_agent_activities(5_000_000);
    let mut group = c.benchmark_group("Activity Loop");
    for num_agents in [1_000u32, 10_000u32, 100_000u32, 500_000u32, 1_000_000, 5_000_000].iter() {
        let mut activities_at_timestep: VecDeque<Vec<Activity>> = VecDeque::new();
        let mut agents_to_activities: Vec<Vec<Activity>> = agents_to_activities_master[0..(*num_agents as usize)].iter().cloned().collect();

        agents_to_activities.iter_mut().for_each(|agent_activities| {
            let activity = agent_activities.pop().unwrap();
            activities_at_timestep.get_mut_or_grow(activity.end_timestep as usize).unwrap().push(activity);
        });

        group.throughput(Throughput::Elements(*num_agents as u64));
        group.bench_with_input(BenchmarkId::new("Sequential", num_agents), num_agents, |b, _| {
            b.iter_batched(
                || (activities_at_timestep.clone(), agents_to_activities.clone()),
                |(activities_at_timestep, agents_to_activities)| { activity_loop(activities_at_timestep, agents_to_activities); },
                BatchSize::LargeInput);
        });
        // group.bench_with_input(BenchmarkId::new("Parallel", num_agents), num_agents, |b, _| {
        //     b.iter_batched(
        //         || (activities_at_timestep.clone(), agents_to_activities.clone()),
        //         |(activities_at_timestep, agents_to_activities)| { activity_loop_parallel(activities_at_timestep, agents_to_activities); },
        //         BatchSize::SmallInput);
        // });
    }
    group.finish();
}

criterion_group! {
    name = benches;
    config = Criterion::default().sample_size(10);
    targets = bench_activity_loop
}
criterion_main!(benches);