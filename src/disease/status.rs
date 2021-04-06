use rand::rngs::StdRng;
use rand::Rng;

// Infection and Disease Progression
#[derive(PartialEq)]
pub enum State {
    Susceptible,
    Infectious,
    Recovered,
}

pub struct DiseaseStatus {
    pub state: State,
    infected_for: u16, // How long the infection has lasted until now / recovery / death
}

impl DiseaseStatus {
    pub fn new(rng: &mut StdRng) -> DiseaseStatus {
        DiseaseStatus {
            state: if rng.gen::<f32>() < 0.9998 { State::Susceptible } else { State::Infectious },
            infected_for: 0
        }
    }

    #[inline]
    pub fn infect(&mut self) {
        debug_assert!(self.state == State::Susceptible);
        self.state = State::Infectious;
        self.infected_for = 0;
    }

    #[inline]
    pub fn progress_infection(&mut self) {
        debug_assert!(self.state == State::Infectious);
        self.infected_for += 1;

        // TODO Update to not be constant
        if self.infected_for > 12 {

        }
    }
}

pub fn construct_disease_status_array(num_agents: u32, rng: &mut StdRng) -> Vec<DiseaseStatus> {
    (0..num_agents).map(|_| DiseaseStatus::new(rng)).collect()
}