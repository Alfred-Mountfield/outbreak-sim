use crate::routing::RoutingType;
use nonmax::NonMaxU64;

#[derive(Debug, Copy, Clone)]
pub struct Event {
    pub agent_idx: u32,
    // start_timestep: u16,
    pub end_timestep: u16,
    pub event_type: EventType,
}

#[derive(Debug, Copy, Clone)]
pub enum EventType {
    EnterContainer(NonMaxU64)
}

// #[derive(Debug, Copy, Clone)]
// pub struct MoveContainer {
//     pub container_idx: NonMaxU64,
//     pub routing_type: RoutingType,
// }

#[inline]
pub fn handle_event(event: Event) {
    match event.event_type {
        EventType::EnterContainer(container_idx) => {

        }
    }
}
