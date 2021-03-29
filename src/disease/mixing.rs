use rand::Rng;

use crate::disease::{DiseaseStatus, State};
use crate::types::TimeStep;

pub trait MixingStrategy<T: Send + Sync = Self>: Send + Sync {
    fn handle_transmission<R>(&self, statuses: &mut [&mut DiseaseStatus], rng: &mut R, for_time_steps: TimeStep)
        where R: Rng + ?Sized;
}

#[derive(Clone)]
pub struct Uniform {
    // Chance an infected person might infect someone else in their container per time step
    pub transmission_chance: f32
}

// pub struct Network {
//
// }

/// Super basic Mixing strategy with a flat exposure rate based on how many infected people are
/// in the container, doesn't take an individual's susceptibility, length of infection, distances,
/// etc. into consideration
impl MixingStrategy for Uniform {
    #[inline]
    fn handle_transmission<R>(&self, statuses: &mut [&mut DiseaseStatus], rng: &mut R, for_time_steps: TimeStep)
        where R: Rng + ?Sized
    {
        let mut num_infected = 0;

        // TODO revisit keeping track of susceptible in this loop, speed was tested for only households which are smaller than workplaces
        for status in statuses.iter_mut() {
            if status.state == State::Infectious {
                num_infected += 1;
                status.progress_infection();
            }
        }

        let chance = self.transmission_chance * (num_infected as f32) * (for_time_steps as f32);
        for agent_status in statuses.iter_mut() {
            if agent_status.state == State::Susceptible && rng.gen::<f32>() < chance {
                agent_status.infect()
            };
        };
    }
}
