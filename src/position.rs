use rand::prelude::StdRng;
use rand::Rng;

pub struct Coord {
    pub x: f32,
    pub y: f32,
}

impl Coord {
    pub fn new(rng: &mut StdRng) -> Coord {
        Coord {
            x: rng.gen(),
            y: rng.gen()
        }
    }

    pub fn update(&mut self, rng: &mut StdRng) {
        let new_x = self.x + (rng.gen_range(-0.005, 0.005));
        if new_x > 1.0 {
            self.x = 0.99;
        } else {
            self.x = new_x;
        }
        let new_y = self.y + (rng.gen_range(-0.005, 0.005));
        if new_y > 1.0 {
            self.y = 0.99;
        } else {
            self.y = new_y;
        }
    }
}


pub fn construct_pos_array(num_agents: u64, mut rng: &mut StdRng) -> Vec<Coord> {
    (0..num_agents).map(|_| Coord::new(&mut rng)).collect()
}
