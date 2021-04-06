use fast_paths::{FastGraph, PathCalculator};
use nonmax::NonMaxU64;
use rand::{Rng, thread_rng};

use crate::agents::Agents;
use crate::containers::Containers;
use crate::disease::MixingStrategy;
use crate::events::event::EventType::{EnterContainer, Travel};
use crate::routing::{calculate_direct_commute_time, calculate_public_transit_commute_time, GranularGrid, RoutingType, DirectRoutingType};
use crate::routing::DirectRoutingType::Driving;
use crate::shared::TIME_STEPS_PER_DAY;
use crate::shared::types::TimeStep;

#[derive(Debug, Copy, Clone)]
pub struct Event {
    pub agent_idx: u32,
    // start_timestep: TimeStep,
    pub end_time_step: TimeStep,
    pub event_type: EventType,
}

#[derive(Debug, Copy, Clone)]
pub enum EventType {
    Travel(TravelType),
    EnterContainer(NonMaxU64),
}

#[derive(Debug, Copy, Clone)]
pub struct TravelType {
    pub from_container_idx: NonMaxU64,
    pub to_container_idx: NonMaxU64,
    pub routing_type: RoutingType,
}

impl Event {
    #[inline]
    pub fn handle<M>(self, agents: &mut Agents, containers: &mut Containers<M>, transit_grid: &GranularGrid<usize>,
                     fast_graph: &FastGraph, transit_path_calculator: &mut PathCalculator) -> Option<Event>
        where M: MixingStrategy {
        match self.event_type {
            EventType::EnterContainer(from_container_idx) => {
                containers.push_inhabitant(from_container_idx.get(), self.agent_idx, self.end_time_step, agents);

                let occupation_container_idx = agents.occupational_container[self.agent_idx as usize].unwrap();
                let to_container_idx = if from_container_idx != occupation_container_idx { occupation_container_idx } else { NonMaxU64::new(agents.household_container[self.agent_idx as usize]).unwrap() };

                Some(Event {
                    agent_idx: self.agent_idx,
                    end_time_step: self.end_time_step + (TIME_STEPS_PER_DAY / 2),
                    event_type: Travel(TravelType {
                        from_container_idx,
                        to_container_idx,
                        routing_type: if thread_rng().gen::<f32>() < 0.8 { RoutingType::Direct(Driving) } else { RoutingType::Transit },
                    }),
                })
            }
            EventType::Travel(travel_type) => {
                containers.remove_inhabitant(travel_type.from_container_idx.get(), self.agent_idx, self.end_time_step, agents);
                let mut commute_time = match travel_type.routing_type {
                    RoutingType::Transit => {
                        if let Ok(commute_time) = calculate_public_transit_commute_time(containers, transit_grid, transit_path_calculator, fast_graph, travel_type.from_container_idx, travel_type.to_container_idx) {
                            commute_time
                        } else {
                            calculate_direct_commute_time(containers, DirectRoutingType::Driving, travel_type.from_container_idx, travel_type.to_container_idx)
                        }
                    }
                    RoutingType::Direct(direct_routing_type) => {
                        calculate_direct_commute_time(containers, direct_routing_type, travel_type.from_container_idx, travel_type.to_container_idx)
                    }
                };

                if commute_time == 0 { commute_time += 1 };
                Some(Event {
                    agent_idx: self.agent_idx,
                    end_time_step: self.end_time_step + commute_time,
                    event_type: EnterContainer(travel_type.to_container_idx),
                })
            }
        }
    }
}
