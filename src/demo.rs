extern crate sdl_rust;

use sdl2::pixels::Color;
use sdl2::rect::Rect;

use sdl_rust::SDLCore;
use sdl_rust::Demo;

use std::time::Duration;
use std::thread;
use sdl2::image::LoadTexture;

const TITLE: &str = "Monster Town Credits Demo";
const CAM_W: u32 = 1280;
const CAM_H: u32 = 720;
const TIMEOUT: u64 = 5000;

const TEAM: &[&str; 6] = &["Title", "Adam", "Azeez", "Burhan", "Gurmail", "Zhiyi"];
const BGS: &[Color; 6] = &[	Color::BLACK,
							Color::RGB(0x91, 0xD5, 0xFF), // Adam
							Color::MAGENTA, // Azeez
							Color::BLUE, // Burhan
							Color::YELLOW, // Gurmail
							Color::RED ]; // Zhiyi

pub struct SDL04 {
	core: SDLCore,
}

impl Demo for SDL04 {
	fn init() -> Result<Self, String> {
		let core = SDLCore::init(TITLE, true, CAM_W, CAM_H)?;
		Ok(SDL04{ core })
	}

	fn run(&mut self) -> Result<(), String> {
		let texture_creator = self.core.wincan.texture_creator();

		let mut i = 0;
		while i < TEAM.len() {
			let member = TEAM[i];

			// Set the background color specified by each member
			let bg_color = BGS[i];
			self.core.wincan.set_draw_color(bg_color);
		
			// Use the image with their name
			let image_path = format!("images/{}.png", member);
			let monster_image = texture_creator.load_texture(image_path)?;
			
			// Get the image dimensions for use in centering it in the display window
			let w = monster_image.query().width;
			let h = monster_image.query().height;

			// Have the image take up the entire height, and be centered horizontally (it will crash for wide images)
			let rect = Rect::new(((CAM_W/2) - (CAM_H * w / h)/2) as i32, 0, CAM_H * w / h, CAM_H);

			// Clear the previous image and set the background color
			self.core.wincan.clear();

			// Add the image to the window (within the specified rectangle)
			self.core.wincan.copy(&monster_image, None, rect)?;

			self.core.wincan.present();

			// Wait for the TIMEOUT (5 seconds), then show the next image
			thread::sleep(Duration::from_millis(TIMEOUT));

			i += 1;
		}
		Ok(())
	}
}

fn main() {
	sdl_rust::runner(TITLE, SDL04::init);
}
