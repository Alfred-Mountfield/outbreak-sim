use rand::Rng;
use rand::rngs::StdRng;

pub struct Coord {
    pub x: f32,
    pub y: f32,
}


impl Coord {
    pub fn new(x: f32, y: f32) -> Coord {
        Coord {
            x,
            y,
        }
    }

    pub fn new_rand(rng: &mut StdRng) -> Coord {
        Coord {
            x: rng.gen(),
            y: rng.gen(),
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