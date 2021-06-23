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

pub struct SDL10 {
	core: SDLCore,
}

impl Demo for SDL10 {
	fn init() -> Result<Self, String> {
		let core = SDLCore::init(TITLE, true, CAM_W, CAM_H)?;
		Ok(SDL10{ core })
	}

	fn run(&mut self) -> Result<(), String> {
		let texture_creator = self.core.wincan.texture_creator();

		let tree_sheet = texture_creator.load_texture("images/tree.png")?;
		let grass_sheet = texture_creator.load_texture("images/grass patch.png")?;

		'gameloop: loop {
			for event in self.core.event_pump.poll_iter() {
				match event {
					Event::Quit{..} | Event::KeyDown{keycode: Some(Keycode::Escape), ..} => break 'gameloop,
					_ => {},
				}
			}

			self.core.wincan.set_draw_color(Color::RGBA(0, 128, 128, 255));
			self.core.wincan.clear();

			// Draw bottom trees
			let mut i = 0;
			while i * TILE_SIZE < CAM_W {
				let src = Rect::new(( (i % 4)  * TILE_SIZE) as i32, 0, TILE_SIZE, 4 * TILE_SIZE);
				let pos = Rect::new((i * TILE_SIZE) as i32, (CAM_H - 4 * TILE_SIZE) as i32, TILE_SIZE, 4 * TILE_SIZE);

				self.core.wincan.copy(&tree_sheet, src, pos)?;

				i += 1;
			}

<<<<<<< HEAD
			// Draw upper trees
			let mut i = 0;
			while i * TILE_SIZE < CAM_W {
				let src = Rect::new(( (i % 4)  * TILE_SIZE) as i32, 0, TILE_SIZE, 4 * TILE_SIZE);
				let pos = Rect::new((i * TILE_SIZE) as i32, 0 , TILE_SIZE, 4 * TILE_SIZE);

				self.core.wincan.copy(&tree_sheet, src, pos)?;

				i += 1;
			}

			// Draw grass patches
			let mut i = 5;
			while i * TILE_SIZE < 320 {
				let src = Rect::new( (i * TILE_SIZE) as i32, 0, TILE_SIZE, TILE_SIZE);
				let pos_1 = Rect::new((i * TILE_SIZE) as i32 , 320 , TILE_SIZE, TILE_SIZE);
				//let pos_2 = Rect::new((i * TILE_SIZE) as i32, 320 + 16, TILE_SIZE, TILE_SIZE);

				self.core.wincan.copy(&grass_sheet, src, pos_1)?;
				//self.core.wincan.copy(&grass_sheet, src, pos_2)?;
				
				i += 1;
=======
			// Draw birds
			for (i, b) in (0..).zip(bird_locations.iter()) {
				let src = Rect::new(
					(i % 2) * (TILE_SIZE as i32),
					((i % 4)/2) * (TILE_SIZE as i32),
					TILE_SIZE,
					TILE_SIZE,
				);
				self.core.wincan.copy(&bird_sheet, src, b.clone())?;
>>>>>>> bfef4e1be03bd06d59d8fe34ad32477cbd95b21a
			}

			self.core.wincan.present();
		}

		// Out of game loop, return Ok
		Ok(())
	}
}

fn main() {
	sdl_rust::runner(TITLE, SDL10::init);
}
