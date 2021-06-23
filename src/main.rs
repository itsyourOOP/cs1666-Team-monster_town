extern crate sdl_rust;
extern crate rand;

use rand::thread_rng;
use rand::Rng;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::rect::Rect;
use sdl2::keyboard::Keycode;
use sdl2::image::LoadTexture;
use sdl2::render::Texture;

use sdl_rust::SDLCore;
use sdl_rust::Demo;

const TITLE: &str = "Map with woods and grass";

const CAM_W: u32 = 1280;
const CAM_H: u32 = 720;

const TILE_SIZE: u32 = 16;

fn main() {
}
