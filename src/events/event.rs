use std::sync::atomic::Ordering;

use nonmax::NonMaxU64;

use crate::agents::Agents;
use crate::disease::MixingStrategy;
use crate::events::event::EventType::{EnterContainer, Travel};
use crate::pois::Containers;
use crate::routing::{calculate_direct_commute_time, RoutingType};
use crate::routing::DirectRoutingType::Driving;
use crate::shared::TIME_STEPS_PER_DAY;

#[derive(Debug, Copy, Clone)]
pub struct Event {
    pub agent_idx: u32,
    // start_timestep: u32,
    pub end_time_step: u32,
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
    pub fn handle<M>(self, agents: &Agents, containers: &mut Containers<M>) -> Option<Event>
        where M: MixingStrategy {
        match self.event_type {
            EventType::EnterContainer(from_container_idx) => {
                containers.push_inhabitant(from_container_idx.get(), self.agent_idx);

                let occupation_container_idx = agents.occupational_container[self.agent_idx as usize].unwrap();
                let to_container_idx = if from_container_idx != occupation_container_idx { from_container_idx } else { NonMaxU64::new(agents.household_container[self.agent_idx as usize]).unwrap() };

                Some(Event {
                    agent_idx: self.agent_idx,
                    end_time_step: self.end_time_step + (TIME_STEPS_PER_DAY.load(Ordering::Relaxed) as u32 / 2),
                    event_type: Travel(TravelType {
                        from_container_idx,
                        to_container_idx,
                        routing_type: RoutingType::Direct(Driving),
                    }),
                })
            }
            EventType::Travel(travel_type) => {
                containers.remove_inhabitant(travel_type.from_container_idx.get(), self.agent_idx);
                match travel_type.routing_type {
                    RoutingType::Transit => { unimplemented!() }
                    RoutingType::Direct(direct_routing_type) => {
                        let commute_time = calculate_direct_commute_time(containers, direct_routing_type, travel_type.from_container_idx, travel_type.to_container_idx);
                        Some(Event {
                            agent_idx: self.agent_idx,
                            end_time_step: self.end_time_step + commute_time,
                            event_type: EnterContainer(travel_type.to_container_idx),
                        })
                    }
                }
            }
        }
    }
}
