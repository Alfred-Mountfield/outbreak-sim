use rand::rngs::StdRng;
use rand::{SeedableRng, Rng};

pub mod graphics;

#[derive(Debug)]
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

pub fn construct_pos_array(num_people: u64) -> Vec<Coord> {
    let mut rng = StdRng::seed_from_u64(32);

    (0..num_people).map(|_| Coord::new(&mut rng)).collect()
    // println!("{:?}", positions);
}
