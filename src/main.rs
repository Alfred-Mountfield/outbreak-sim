use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::{LogicalSize};
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit_input_helper::WinitInputHelper;
use winit::window::WindowBuilder;
use outbreak_sim::{agents, read_buffer, get_root_as_model};

const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 800;
const WORLD_WIDTH: u32 = 200;
const WORLD_HEIGHT: u32 = 200;

mod graphics;


fn main() -> Result<(), Error> {
    let bytes = read_buffer("python/synthetic_population/output/model_tower_hamlets_150k.txt");
    let model = get_root_as_model(&bytes);
    let household_positions = model.households().pos();
    let agents = agents::Agents::new(household_positions);
    // println!("{}", model.households().pos()[0].x());
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
            world.update(&agents);
            window.request_redraw();
        }
    });
}
