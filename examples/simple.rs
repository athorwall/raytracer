extern crate sdl2;
extern crate raytracer;

use raytracer::sdl::*;

fn main() {
    let ctx = sdl2::init().unwrap();
    let canvas = create_sdl_canvas(&ctx, 1000, 800);
}