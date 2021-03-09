use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::{LogicalSize};
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit_input_helper::WinitInputHelper;
use winit::window::WindowBuilder;
use outbreak_sim::{routing, agents, read_buffer, get_root_as_model};
use std::time::Instant;
use rand::{Rng, SeedableRng};
use rayon::prelude::*;
use fast_paths::ShortestPath;
use outbreak_sim::routing::{nodes_to_granular_grid, sample_nearby_from_grid};
use rand::rngs::StdRng;
use rand::seq::SliceRandom;

const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 800;
const WORLD_WIDTH: u32 = 600;
const WORLD_HEIGHT: u32 = 600;

mod graphics;


fn main() -> Result<(), Error> {
    let bytes = read_buffer("python/synthetic_population/output/model_greater_manchester_1m.txt");
    // let bytes = read_buffer("python/synthetic_population/output/model_london_se_commuter_ring_8m.txt");
    let model = get_root_as_model(&bytes);
    // TODO Ensure that this is non-inclusive
    let bounds = model.bounds().to_owned(); // TODO Ensure that min is (0,0) or handle otherwise

    let agent_households = model.agents().household_index();
    let household_positions = model.households().pos();
    let agents = agents::Agents::new(agent_households, household_positions);

    let transit_graph = model.transit_graph();

    println!("Creating Contraction Hierarchies");
    let now = Instant::now();
    let fast_graph = routing::preprocess_graph(&transit_graph);
    println!("{:.6}", now.elapsed().as_secs_f64());

    println!("Creating Granular Grid of Transit Nodes");
    let now = Instant::now();
    let transit_node_grid = nodes_to_granular_grid(&transit_graph, &bounds, 50);
    println!("{:.6}", now.elapsed().as_secs_f64());

    let workplace_indices = model.agents().workplace_index().unwrap().safe_slice();
    let workplace_positions = model.workplaces().pos();

    println!("Finding all transit nodes at cell for each person sequentially");
    let now = Instant::now();
    let mut rng = rand::thread_rng();
    agents.positions.iter().zip(workplace_indices.iter()).for_each(|(pos, workplace_idx)| {
        if *workplace_idx != u32::MAX {
            transit_node_grid[[pos.x(), pos.y()]].choose(&mut rng);
        }
    });
    drop(now);
    println!("{:.6}", now.elapsed().as_secs_f64());

    println!("Finding all transit nodes at cell for each person in parallel");
    let now = Instant::now();
    agents.positions.par_iter().zip(workplace_indices.par_iter())
        .for_each_init(
            || rand::thread_rng(),
            |mut rng, (pos, workplace_idx)| {
                if *workplace_idx != u32::MAX {
                    transit_node_grid[[pos.x(), pos.y()]].choose(&mut rng);
                }
    });
    println!("{:.6}", now.elapsed().as_secs_f64());

    println!("Choosing a transit node for each person sequentially");
    let now = Instant::now();
    let mut rng = rand::thread_rng();
    agents.positions.iter().zip(workplace_indices.iter()).for_each(|(pos, workplace_idx)| {
        if *workplace_idx != u32::MAX {
            let src_node = sample_nearby_from_grid(&transit_node_grid, (pos.x(), pos.y()), 2_000.0, &mut rng).unwrap();
        }
    });
    println!("{:.6}", now.elapsed().as_secs_f64());
    drop(now);

    println!("Choosing a transit node for each person in parallel");
    let now = Instant::now();
    agents.positions.par_iter().zip(workplace_indices.par_iter())
        .for_each_init(
            || rand::thread_rng(),
            |mut rng, (pos, workplace_idx)| {
                if *workplace_idx != u32::MAX {
                    let src_node = sample_nearby_from_grid(&transit_node_grid, (pos.x(), pos.y()), 2_000.0, &mut rng).unwrap();
                }
            });
    println!("{:.6}", now.elapsed().as_secs_f64());

    println!("Calculating all workplace public transit commutes sequentially");
    let now = Instant::now();
    let mut rng = rand::thread_rng();
    let mut path_calculator = fast_paths::create_calculator(&fast_graph);

    agents.positions.iter().zip(workplace_indices.iter())
        .for_each(
            |(pos, workplace_idx)| {
                if *workplace_idx != u32::MAX {
                    let src_node = sample_nearby_from_grid(&transit_node_grid, (pos.x(), pos.y()), 2_000.0, &mut rng).unwrap();
                    let workplace_position = workplace_positions[*workplace_idx as usize];
                    let dest_node = sample_nearby_from_grid(&transit_node_grid, (workplace_position.x(), workplace_position.y()), 2_000.0, &mut rng).unwrap();

                    path_calculator.calc_path(&fast_graph, *src_node, *dest_node);
                }
            });
    println!("{:.6}", now.elapsed().as_secs_f64());
    drop(rng); drop(path_calculator);

    println!("Calculating all workplace public transit commutes in parallel");
    let now = Instant::now();
    agents.positions.par_iter().zip(workplace_indices.par_iter())
        .for_each_init(
            || (rand::thread_rng(), fast_paths::create_calculator(&fast_graph)),
            |(mut rng, path_calculator), (pos, workplace_idx)| {
                if *workplace_idx != u32::MAX {
                    let src_node = sample_nearby_from_grid(&transit_node_grid, (pos.x(), pos.y()), 2_000.0, &mut rng).unwrap();
                    let workplace_position = workplace_positions[*workplace_idx as usize];
                    let dest_node = sample_nearby_from_grid(&transit_node_grid, (workplace_position.x(), workplace_position.y()), 2_000.0, &mut rng).unwrap();

                    path_calculator.calc_path(&fast_graph, *src_node, *dest_node);
                }
            });
    println!("{:.6}", now.elapsed().as_secs_f64());
    drop(rng);


    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let window = {
        let size = LogicalSize::new(SCREEN_WIDTH as f64, SCREEN_HEIGHT as f64);
        WindowBuilder::new()
            .with_title("Outbreak Sim")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(WORLD_WIDTH as u32, WORLD_WIDTH as u32, surface_texture)?
    };
    let mut world = graphics::WorldGrid::new_empty(WORLD_HEIGHT as usize, WORLD_WIDTH as usize);

    event_loop.run(move |event, _, control_flow| {
        // Draw the current frame
        if let Event::RedrawRequested(_) = event {
            world.draw(pixels.get_frame());
            if pixels
                .render()
                .map_err(|e| ("pixels.render() failed: {}", e))
                .is_err()
            {
                *control_flow = ControlFlow::Exit;
                return;
            }
        }

        // Handle input events
        if input.update(&event) {
            // Close events
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            // Resize the window
            if let Some(size) = input.window_resized() {
                pixels.resize(size.width, size.height);
            }

            // Update internal state and request a redraw
            // agents.update();
            world.update(&agents, &bounds);
            window.request_redraw();
        }
    });
}
