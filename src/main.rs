mod shader;
mod camera;
mod utils;
mod window;
mod model;
mod renderer;
mod material;
mod assets_manager;

extern crate sdl2;
extern crate gl;

use crate::window::RatWindow;

fn main() {
    let mut window = RatWindow::new(800, 600);
    window.run_loop();
}