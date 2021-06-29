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

const TITLE: &str = "Monster Town Week 2";
const TILE_SIZE: u32 = 16;

// Camera
const CAM_W: u32 = 1280;
const CAM_H: u32 = 720;
//Background
const BG_W: u32 = 1280;
const BG_H: u32 = 720;

const MAX_SPEED: i32 = 5;
const ACCEL_RATE: i32 = 1;

// Im not sure what these are used for
const SCALE_UP: i16 = 3;
const HELP_WHERE_DOES_THIS_COME_FROM: i32 = 2086;

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
    let texture_creator = self.core.wincan.texture_creator();

    //Commented old town map. Remove later.
    //let background_image = texture_creator.load_texture("images/MapHolder.png")?;

    // Texture
    let texture_creator = self.core.wincan.texture_creator();

    let tree_sheet = texture_creator.load_texture("images/tree.png")?;
    let grass_sheet = texture_creator.load_texture("images/grass_patch_32.png")?;
    let water_sheet = texture_creator.load_texture("images/water_patch_32.png")?;
    let gym = texture_creator.load_texture("images/gymA.png")?;

    // Player Creation from mod player.rs
    let mut p = Player::create(
      Rect::new(
        0,
        0,
        TILE_SIZE * SCALE_UP as u32,
        TILE_SIZE * SCALE_UP as u32,
      ),
      texture_creator.load_texture("images/Character.png")?,
    );

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
      let mut i = 58;
      while i * TILE_SIZE < 1184 {
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

      self.core.wincan.present();

      // Create the Town Gym
      let gym_box = Rect::new(300, 100, 150, 150);
      self.core.wincan.copy(&gym, None, gym_box)?;
      self.core.wincan.present();

      let mut movement_direction;
      let mut speed_update;

      // Implement Keystate
      let keystate: HashSet<Keycode> = self
        .core
        .event_pump
        .keyboard_state()
        .pressed_scancodes()
        .filter_map(Keycode::from_scancode)
        .collect();

      if keystate.contains(&Keycode::W) || keystate.contains(&Keycode::Up) {
        movement_direction = 1;
      } else if keystate.contains(&Keycode::A) || keystate.contains(&Keycode::Left) {
        movement_direction = 2;
      } else if keystate.contains(&Keycode::S) || keystate.contains(&Keycode::Down) {
        movement_direction = 3;
      } else if keystate.contains(&Keycode::D) || keystate.contains(&Keycode::Right) {
        movement_direction = 4;
      } else {
        movement_direction = 0;
      }

      self.core.wincan.clear();

      let x_limits = (0, HELP_WHERE_DOES_THIS_COME_FROM as i32);

      match movement_direction {
        1 => {
          speed_update = (0, -MAX_SPEED as i32);
        }
        2 => {
          speed_update = (-MAX_SPEED as i32, 0);
        }
        3 => {
          speed_update = (0, MAX_SPEED as i32);
        }
        4 => {
          speed_update = (MAX_SPEED as i32, 0);
        }
        _ => {
          speed_update = (0, 0);
        }
      }
      p.update_pos(
        speed_update,
        x_limits,
        (
          0,
          ((BG_H + (SCALE_UP * TILE_SIZE as i16) as u32) * (SCALE_UP as u32) / 2) as i32,
        ),
      );

      let mut player_box = Rect::new(p.x(), p.y(), p.height(), p.width());
      if check_collision(&player_box, &gym_box) {
        p.set_x(p.x() - speed_update.0);
      }
      if check_collision(&player_box, &gym_box) {
        p.set_y(p.y() - speed_update.1);
      }

      // Determine the current portion of the background to draw
      let cur_bg = Rect::new(
        ((p.x() + ((p.width() / 2) as i32)) - ((CAM_W / 2) as i32))
          .clamp(0, (BG_W - (CAM_W / SCALE_UP as u32)) as i32),
        ((p.y() + ((p.height() / 2) as i32)) - ((CAM_H / 2) as i32))
          .clamp(0, (BG_H - (CAM_H / SCALE_UP as u32)) as i32),
        CAM_W / SCALE_UP as u32,
        CAM_H / SCALE_UP as u32,
      );

      // Convert player's map position to be camera-relative
      let player_cam_pos = Rect::new(
        p.x() - cur_bg.x(),
        p.y() - cur_bg.y(),
        TILE_SIZE * SCALE_UP as u32,
        TILE_SIZE * SCALE_UP as u32,
      );

      //self.core.wincan.copy(&tree_sheet, cur_bg, None)?;
      self.core.wincan.copy(p.texture(), None, player_cam_pos)?;

      self.core.wincan.present();
    }

    Ok(())
  }
}

fn main() {
  sdl_rust::runner(TITLE, SDL04::init);
}
