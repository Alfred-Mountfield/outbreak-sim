use rand::Rng;

use crate::disease::{DiseaseStatus, State};
use crate::disease::status::State::Susceptible;
use crate::agents::Agents;

pub trait MixingStrategy {
    fn handle_transmission<R>(&self, agents: &mut Agents, agent_indices: &[u32], rng: &mut R)
        where R: Rng + ?Sized;
}

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
    fn handle_transmission<R>(&self, agents: &mut Agents, agent_indices: &[u32], rng: &mut R)
        where R: Rng + ?Sized
    {
        let mut num_infected: u16 = 0;
        let mut count = 0;

        // let inhabitants = agent_indices.iter()
        //     .for_each(|&agent_index| {
        //         let status = &mut agents.disease_statuses[agent_index as usize];
        //         if status.state == State::Infectious { count += 1 }
        //
        //     });
        //
        // let mut susceptible: Vec<&mut DiseaseStatus> = inhabitants.into_iter()
        //     .inspect(|&status| )
        //     .filter(|)
        //     .collect();
        //
        // let chance = self.transmission_chance * (num_infected as f32);
        // for agent_status in susceptible.iter_mut() {
        //     if rng.gen::<f32>() > chance { agent_status.infect() };
        // };
    }
}
