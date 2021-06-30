extern crate sdl_rust;

// Modules
mod player;
use player::Player;

use sdl2::event::Event;
use sdl2::image::LoadTexture;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Texture;
use sdl_rust::Demo;
use sdl_rust::SDLCore;
use std::collections::HashSet;

const TITLE: &str = "Monster Town Week 3";
const TILE_SIZE: u32 = 16;

// Camera
const CAM_W: u32 = 1280;
const CAM_H: u32 = 720;

const MAX_SPEED: i32 = 5;
const ACCEL_RATE: i32 = 1;

const SCALE_UP: i16 = 3;

pub struct SDL04 {
  core: SDLCore,
}
fn resist(vel: i32, deltav: i32) -> i32 {
  if deltav == 0 {
    if vel > 0 {
      -1
    } else if vel < 0 {
      1
    } else {
      deltav
    }
  } else {
    deltav
  }
}

fn check_collision(a: &Rect, b: &Rect) -> bool {
  if a.bottom() < b.top() || a.top() > b.bottom() || a.right() < b.left() || a.left() > b.right() {
    false
  } else {
    true
  }
}

impl Demo for SDL04 {
  fn init() -> Result<Self, String> {
    let core = SDLCore::init(TITLE, true, CAM_W, CAM_H)?;
    Ok(SDL04 { core })
  }

  fn run(&mut self) -> Result<(), String> {

    // Texture
    let texture_creator = self.core.wincan.texture_creator();

    let tree_sheet = texture_creator.load_texture("images/tree.png")?;
    let grass_sheet = texture_creator.load_texture("images/grass_patch_32.png")?;
    let water_sheet = texture_creator.load_texture("images/water_patch_32.png")?;
    let gym_1 = texture_creator.load_texture("images/GymV6.png")?;
    let gym_2 = texture_creator.load_texture("images/GymV7.png")?;

    let mut x_vel = 0;
    let mut y_vel = 0;

    // Player Creation from mod player.rs
    // it has a start position
    let mut player = Player::create(
      Rect::new(
        64,
        64,
        TILE_SIZE * 2 as u32,
        TILE_SIZE * 2 as u32,
      ),
      texture_creator.load_texture("images/walk1_32.png")?,
    );

	let mut player_box = Rect::new(player.x(), player.y(), player.height(), player.width());

    'gameloop: loop {
      for event in self.core.event_pump.poll_iter() {
        match event {
          Event::Quit { .. }
          | Event::KeyDown {
            keycode: Some(Keycode::Escape),
            ..
          } => break 'gameloop,
          _ => {}
        }
      }

      self
        .core
        .wincan
        .set_draw_color(Color::RGBA(0, 128, 128, 255));
      self.core.wincan.clear();

      // Draw bottom trees
      let mut i = 0;
      while i * TILE_SIZE < CAM_W {
        let src = Rect::new(((i % 4) * TILE_SIZE) as i32, 0, TILE_SIZE, 4 * TILE_SIZE);
        let pos = Rect::new(
          (i * TILE_SIZE) as i32,
          (CAM_H - 4 * TILE_SIZE) as i32,
          TILE_SIZE,
          4 * TILE_SIZE,
        );

        self.core.wincan.copy(&tree_sheet, src, pos)?;

        i += 1;
      }

      // Draw upper trees
      let mut i = 0;
      while i * TILE_SIZE < CAM_W {
        let src = Rect::new(((i % 4) * TILE_SIZE) as i32, 0, TILE_SIZE, 4 * TILE_SIZE);
        let pos = Rect::new((i * TILE_SIZE) as i32, 0, TILE_SIZE, 4 * TILE_SIZE);

        self.core.wincan.copy(&tree_sheet, src, pos)?;

        i += 1;
      }

      // Draw grass patches
      let mut i = 6;
      while i * TILE_SIZE < 320 {
        let src = Rect::new(((i % 2) * TILE_SIZE) as i32, 0, TILE_SIZE, 2 * TILE_SIZE);
        let pos_1 = Rect::new((i * TILE_SIZE) as i32, 96, TILE_SIZE, 2 * TILE_SIZE);
        let pos_2 = Rect::new((i * TILE_SIZE) as i32, 128, TILE_SIZE, 2 * TILE_SIZE);
        let pos_3 = Rect::new((i * TILE_SIZE) as i32, 160, TILE_SIZE, 2 * TILE_SIZE);
        let pos_4 = Rect::new((i * TILE_SIZE) as i32, 192, TILE_SIZE, 2 * TILE_SIZE);

        self.core.wincan.copy(&grass_sheet, src, pos_1)?;
        self.core.wincan.copy(&grass_sheet, src, pos_2)?;
        self.core.wincan.copy(&grass_sheet, src, pos_3)?;
        self.core.wincan.copy(&grass_sheet, src, pos_4)?;

        i += 1;
      }

      // Draw a pond
      let mut i = 48;
      while i * TILE_SIZE < 1060 {
        let src = Rect::new(((i % 2) * TILE_SIZE) as i32, 0, TILE_SIZE, 2 * TILE_SIZE);
        let pos_1 = Rect::new((i * TILE_SIZE) as i32, 480, TILE_SIZE, 2 * TILE_SIZE);
        let pos_2 = Rect::new((i * TILE_SIZE) as i32, 514, TILE_SIZE, 2 * TILE_SIZE);
        let pos_3 = Rect::new((i * TILE_SIZE) as i32, 546, TILE_SIZE, 2 * TILE_SIZE);
        let pos_4 = Rect::new((i * TILE_SIZE) as i32, 578, TILE_SIZE, 2 * TILE_SIZE);

        self.core.wincan.copy(&water_sheet, src, pos_1)?;
        self.core.wincan.copy(&water_sheet, src, pos_2)?;
        self.core.wincan.copy(&water_sheet, src, pos_3)?;
        self.core.wincan.copy(&water_sheet, src, pos_4)?;

        i += 1;
      }

      // Create the Town Gym
      let gym_1_box = Rect::new(340, 90, 150, 150);
      self.core.wincan.copy(&gym_1, None, gym_1_box)?;
      // Create Second Town Gym

      let gym_2_box = Rect::new(1110, 450, 150, 150);
      self.core.wincan.copy(&gym_2, None, gym_2_box)?;

      //let mut movement_direction;
      //let mut speed_update;

      // Implement Keystate
      let keystate: HashSet<Keycode> = self
        .core
        .event_pump
        .keyboard_state()
        .pressed_scancodes()
        .filter_map(Keycode::from_scancode)
        .collect();

      let mut x_deltav = 0;
      let mut y_deltav = 0;
      
      if keystate.contains(&Keycode::W) {
            y_deltav -= ACCEL_RATE;
      } 
      if keystate.contains(&Keycode::A) {
            x_deltav -= ACCEL_RATE;
      } 
      if keystate.contains(&Keycode::S) {
            y_deltav += ACCEL_RATE;
      } 
      if keystate.contains(&Keycode::D) {
            x_deltav += ACCEL_RATE;
      } 

      //Utilize the resist function: slowing it down
      x_deltav = resist(x_vel, x_deltav);
      y_deltav = resist(y_vel, y_deltav);

      //self.core.wincan.clear();

      // not exceed speed limit
      x_vel = (x_vel + x_deltav).clamp(-MAX_SPEED,MAX_SPEED);
      y_vel = (y_vel + y_deltav).clamp(-MAX_SPEED,MAX_SPEED);
     
      // Try to move horizontally
      player_box.set_x(player_box.x() + x_vel);
      // Check for collision between player and gyms as well as cam bounds
      // Use the "go-back" approach to collision resolution
      if check_collision(&player_box, &gym_1_box)
          || check_collision(&player_box, &gym_2_box)
          || player_box.left() < 0
          || player_box.right() > CAM_W as i32
      {
          player_box.set_x(player_box.x() - x_vel);
      }

      // Try to move vertically
      player_box.set_y(player_box.y() + y_vel);
      // Check for collision between player and gyms as well as cam bounds(need to consider trees)
      // Use the "go-back" approach to collision resolution
      if check_collision(&player_box, &gym_1_box)
          || check_collision(&player_box, &gym_2_box)
          || player_box.top() < 64
          || player_box.bottom() > CAM_H as i32 - 64
      {
          player_box.set_y(player_box.y() - y_vel);
      }

      //self.core.wincan.copy(&tree_sheet, cur_bg, None)?;
      self.core.wincan.copy(player.texture(), None, player_box)?;

      self.core.wincan.present();
    }

    Ok(())
  }
}

fn main() {
  sdl_rust::runner(TITLE, SDL04::init);
}
