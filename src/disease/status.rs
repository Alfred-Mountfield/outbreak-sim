use rand::rngs::StdRng;
use rand::Rng;
use crate::shared::{TIME_STEPS_PER_DAY, SEED_INFECTION_CHANCE};
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
    pub fn new(rng: &mut StdRng) -> DiseaseStatus {
        if rng.gen::<f32>() > SEED_INFECTION_CHANCE {
            DiseaseStatus {
                state: State::Susceptible,
                infected_for: 0,
            }
        } else {
            DiseaseStatus {
                state: State::Infectious,
                infected_for: rng.gen_range(0..3),
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
        if self.infected_for > 12 * TIME_STEPS_PER_DAY {
            self.state = State::Recovered
        }
        else if self.infected_for > 3 * TIME_STEPS_PER_DAY {
            self.state = State::Infectious
        }
    }
}

pub fn construct_disease_status_array(num_agents: u32, rng: &mut StdRng) -> Vec<DiseaseStatus> {
    (0..num_agents).map(|_| DiseaseStatus::new(rng)).collect()
}