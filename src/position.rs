use rand::prelude::StdRng;
use rand::Rng;
use std::path::Path;
use std::io::{BufReader, BufRead};
use std::fs::File;

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

pub fn construct_pos_array_from_txt(path: &Path) -> Vec<Coord> {
    let br = BufReader::new(File::open(path).expect("failed to read pos txt file"));

    let mut height: u32 = 0;
    let v: Vec<i64> = br.lines().into_iter().flat_map(|line| {
        height += 1;
        line.unwrap().split(' ')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .map(|s| s.parse::<i64>().unwrap())
            .collect::<Vec<i64>>()
    }).collect();

    let width = v.len() / height as usize;

    v.into_iter().enumerate().flat_map(|(vec_index, num_people)| {
        let x = ((vec_index % width) as f32) / width as f32;
        let y = ((vec_index / width) as f32) / height as f32;

        (0..num_people).map(move |_| {
            Coord::new(x, y)
        })
    }).collect()
}


pub fn construct_random_pos_array(num_agents: u64, mut rng: &mut StdRng) -> Vec<Coord> {
    (0..num_agents).map(|_| Coord::new_rand(&mut rng)).collect()
}
