mod shader;
mod camera;
mod utils;
mod window;
mod model;
mod renderer;
mod material;
mod assets_manager;
mod rat_cfg;
mod object;
mod engine;

extern crate sdl2;
extern crate gl;

use crate::engine::engine;

fn main() {
    engine().run();
}