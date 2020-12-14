// use rand::prelude::StdRng;
// use rand::Rng;
// use std::path::Path;
// use std::io::{BufReader, BufRead};
// use std::fs::File;
//

// pub fn construct_pos_array_from_txt(path: &Path) -> Vec<Coord> {
//     let br = BufReader::new(File::open(path).expect("failed to read pos txt file"));
//
//     let mut height: u32 = 0;
//     let v: Vec<i64> = br.lines().into_iter().flat_map(|line| {
//         height += 1;
//         line.unwrap().split(' ')
//             .map(|s| s.trim())
//             .filter(|s| !s.is_empty())
//             .map(|s| s.parse::<i64>().unwrap())
//             .collect::<Vec<i64>>()
//     }).collect();
//
//     let width = v.len() / height as usize;
//
//     v.into_iter().enumerate().flat_map(|(vec_index, num_people)| {
//         let x = ((vec_index % width) as f32) / width as f32;
//         let y = ((vec_index / width) as f32) / height as f32;
//
//         (0..num_people).map(move |_| {
//             Coord::new(x, y)
//         })
//     }).collect()
// }
//
//
// pub fn construct_random_pos_array(num_agents: u64, mut rng: &mut StdRng) -> Vec<Coord> {
//     (0..num_agents).map(|_| Coord::new_rand(&mut rng)).collect()
// }