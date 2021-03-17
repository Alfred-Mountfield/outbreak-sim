use rand::Rng;

use crate::disease::{DiseaseStatus, State};
use crate::disease::status::State::Susceptible;
use crate::agents::Agents;
use crate::pois::Reuse;

pub trait MixingStrategy<T: Send + Sync = Self>: Send + Sync {
    fn handle_transmission<R>(&self, statuses: &mut [&mut DiseaseStatus], rng: &mut R, reuse: &mut Reuse<DiseaseStatus>)
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
    fn handle_transmission<R>(&self, statuses: &mut [&mut DiseaseStatus], rng: &mut R, reuse: &mut Reuse<DiseaseStatus>)
        where R: Rng + ?Sized
    {
        let mut num_infected = 0;
        let mut tmp = reuse.take();

        for status in statuses.iter_mut() {
            if status.state == State::Infectious {
                num_infected += 1;
                status.progress_infection();
            }
            else if status.state == State::Susceptible {
                tmp.push(status);
            }
        }

        let chance = self.transmission_chance * (num_infected as f32);
        for agent_status in tmp.drain(..) {
            if rng.gen::<f32>() < chance {
                agent_status.infect()
            };
        };

        reuse.put(tmp);
    }
}
