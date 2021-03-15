use rand::Rng;

use crate::disease::{DiseaseStatus, State};
use crate::disease::status::State::Susceptible;

pub trait MixingStrategy {
    fn handle_transmission<R>(&self, agent_statuses: &mut [DiseaseStatus], rng: &mut R)
        where R: Rng + ?Sized;
}

pub struct Uniform {
    // Chance an infected person might infect someone else in their container
    transmission_chance: f32
}

// pub struct Network {
//
// }

/// Super basic Mixing strategy with a flat exposure rate based on how many infected people are
/// in the container, doesn't take an individual's susceptibility, length of infection, distances,
/// etc. into consideration
impl MixingStrategy for Uniform {
    #[inline]
    fn handle_transmission<R>(&self, agent_statuses: &mut [DiseaseStatus], rng: &mut R)
        where R: Rng + ?Sized
    {
        let mut num_infected: u16 = 0;

        let mut count = 0;
        let mut susceptible: Vec<&mut DiseaseStatus> = agent_statuses.iter_mut()
            .inspect(|status| if status.state == State::Infectious { count += 1 })
            .filter(|status| status.state == State::Susceptible)
            .collect();

        // let susceptible = agent_statuses.split_mut(|agent_status| {
        //     match agent_status.state {
        //         State::Susceptible => { true }
        //         State::Infectious => {
        //             num_infected += 1;
        //             return false;
        //         }
        //         State::Recovered => { true }
        //     }
        // });

        let chance = self.transmission_chance * (num_infected as f32);
        for agent_status in susceptible.iter_mut() {
            if rng.gen::<f32>() > chance { agent_status.infect() };
        };
    }
}
