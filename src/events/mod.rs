use rand::{Rng, thread_rng};

use crate::agents::Agents;
use crate::disease::MixingStrategy;
use crate::events::event::{Event, EventType};
use crate::events::event_index::{EventIndex, Update, VecDequeMutExt};
use crate::containers::Containers;
use crate::shared::TIME_STEPS_PER_DAY;
use crate::shared::types::TimeStep;
use fast_paths::{PathCalculator, FastGraph};
use crate::routing::GranularGrid;

mod event;
mod event_index;

pub struct Events {
    event_index: EventIndex,
}

impl Events {
    pub fn new(agents: &mut Agents) -> Self {
        let mut rng = thread_rng();
        let mut event_index = EventIndex::default();

        agents.occupational_container.iter()
            .enumerate()
            .filter(|(_, &container_idx)| container_idx.is_some())
            .map(|(agent_idx, container_idx)| {
                Event {
                    agent_idx: agent_idx as u32,
                    end_time_step: tmp_weighted_commute_time(&mut rng),
                    event_type: EventType::EnterContainer(container_idx.unwrap())
                }
            })
            .for_each(|event| {
                event_index.get_mut_or_grow(event.end_time_step as usize).unwrap().push(event);
            });

        Events {
            event_index
        }
    }

    pub fn update<M>(&mut self, time_step: TimeStep, agents: &mut Agents, containers: &mut Containers<M>, transit_grid: &GranularGrid<usize>, 
                     fast_graph: &FastGraph, transit_path_calculator: &mut PathCalculator) where M: MixingStrategy {
        self.event_index.update(time_step, agents, containers, transit_grid, fast_graph, transit_path_calculator);
    }
}

// hacky, unsupported, attempt to get some form of distributed commuting
#[inline]
fn tmp_weighted_commute_time<R>(rng: &mut R) -> TimeStep
    where R: Rng + ?Sized
{
    let time_steps_per_hour: TimeStep = TIME_STEPS_PER_DAY / 24;
    // commute start times range from 4am to 11am
    let earliest = 4 * time_steps_per_hour;
    let time_steps_range = 7 * time_steps_per_hour;

    earliest + (rng.gen::<f32>() * time_steps_range as f32) as TimeStep
}