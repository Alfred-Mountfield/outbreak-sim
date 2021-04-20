use rand::Rng;

use crate::shared::{get_seed_infection_chance, get_time_steps_per_day};
use crate::shared::types::TimeStep;

// Infection and Disease Progression
#[derive(PartialEq, Clone, Copy)]
pub enum State {
    Susceptible,
    Exposed,
    Infectious,
    Recovered,
}

#[derive(Clone, Copy)]
pub struct DiseaseStatus {
    pub state: State,
    infected_for: TimeStep, // How long the infection has lasted until now / recovery / death
}

impl DiseaseStatus {
    pub fn new<R>(rng: &mut R) -> DiseaseStatus
        where R: Rng + ?Sized
    {
        if rng.gen::<f32>() > get_seed_infection_chance() {
            DiseaseStatus {
                state: State::Susceptible,
                infected_for: 0,
            }
        } else if rng.gen::<f32>() < 0.4 {
            DiseaseStatus {
                state: State::Exposed,
                infected_for: (rng.gen_range((0.0)..(2.0)) * get_time_steps_per_day() as f32) as TimeStep,
            }
        } else {
            DiseaseStatus {
                state: State::Infectious,
                infected_for: (rng.gen_range((2.0)..(12.0)) * get_time_steps_per_day() as f32) as TimeStep,
            }
        }
    }

    #[inline]
    pub fn infect(&mut self) {
        debug_assert!(self.state == State::Susceptible);
        self.state = State::Exposed;
        self.infected_for = 0;
    }

    #[inline]
    pub fn progress_infection(&mut self, time_steps: TimeStep) {
        debug_assert!(self.state == State::Exposed || self.state == State::Infectious);
        self.infected_for += time_steps;

        // TODO Update to not be constant
        if self.infected_for > 12 * get_time_steps_per_day() {
            self.state = State::Recovered
        }
        else if self.infected_for > 3 * get_time_steps_per_day() {
            self.state = State::Infectious
        }
    }
}

pub fn construct_disease_status_array<R>(num_agents: u32, rng: &mut R) -> Vec<DiseaseStatus>
    where R: Rng + ?Sized
{
    (0..num_agents).map(|_| DiseaseStatus::new(rng)).collect()
}