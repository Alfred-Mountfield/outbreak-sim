use rand::rngs::StdRng;
use rand::{SeedableRng, Rng};

pub mod graphics;

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
}

pub struct People {
    pub positions: Vec<Coord>,
    rng: StdRng
}

impl People {
    pub fn new(num_people: u64) -> People {
        let mut rng = StdRng::seed_from_u64(32);
        People {
            positions: construct_pos_array(num_people, &mut rng),
            rng: rng
        }
    }

    pub fn update(&mut self) {
        for coord in self.positions.iter_mut() {
            let new_x = coord.x + (self.rng.gen_range(-0.5, 0.5));
            if new_x > 1.0 {
                coord.x = 0.99;
            } else {
                coord.x = new_x;
            }
            let new_y = coord.y + (self.rng.gen_range(-0.5, 0.5));
            if new_y > 1.0 {
                coord.y = 0.99;
            } else {
                coord.y = new_y;
            }
        }
    }
}


fn construct_pos_array(num_people: u64, mut rng: &mut StdRng) -> Vec<Coord> {
    (0..num_people).map(|_| Coord::new(&mut rng)).collect()
}

