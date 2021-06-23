extern crate sdl_rust;

use sdl_rust::Demo;
use sdl_rust::SDLCore;

use std::collections::HashSet;

use sdl2::event::Event;
use sdl2::image::LoadTexture;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use sdl2::render::Texture;

const TITLE: &str = "Monster Town Week 2";

const SCALE_UP: i16 = 3;

const CAM_W: u32 = 1280;
const CAM_H: u32 = 720;

const HELP_WHERE_DOES_THIS_COME_FROM: i32 = 2086;

const SPEED: i8 = 5;

const BG_W: u32 = 1280;
const BG_H: u32 = 720;

const TILE_SIZE: u32 = 16;

pub struct SDL04 {
  core: SDLCore,
}

struct Player<'a> {
  pos: Rect,
  texture: Texture<'a>,
}

impl<'a> Player<'a> {
  fn new(pos: Rect, texture: Texture<'a>) -> Player {
    Player { pos, texture }
  }

  fn x(&self) -> i32 {
    self.pos.x()
  }
  fn y(&self) -> i32 {
    self.pos.y()
  }
  fn width(&self) -> u32 {
    self.pos.width()
  }
  fn height(&self) -> u32 {
    self.pos.height()
  }

  fn update_pos(&mut self, vel: (i32, i32), x_bounds: (i32, i32), y_bounds: (i32, i32)) {
    self
      .pos
      .set_x((self.pos.x() + vel.0).clamp(x_bounds.0, x_bounds.1));
    self
      .pos
      .set_y((self.pos.y() + vel.1).clamp(y_bounds.0, y_bounds.1));
  }

  fn texture(&self) -> &Texture {
    &self.texture
  }
}

impl Demo for SDL04 {
  fn init() -> Result<Self, String> {
    let core = SDLCore::init(TITLE, true, CAM_W, CAM_H)?;
    Ok(SDL04 { core })
  }

  fn run(&mut self) -> Result<(), String> {
    let texture_creator = self.core.wincan.texture_creator();
    let background_image = texture_creator.load_texture("images/MapHolder.png")?;

    let mut movement_direction;
    let mut speed_update;

    let w = 10;
    let static_box = Rect::new((CAM_W / 2 + 2 * w) as i32, (CAM_H / 2 - w / 2) as i32, w, w);

    let mut p = Player::new(
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
          speed_update = (0, -SPEED as i32);
        }
        2 => {
          speed_update = (-SPEED as i32, 0);
        }
        3 => {
          speed_update = (0, SPEED as i32);
        }
        4 => {
          speed_update = (SPEED as i32, 0);
        }
        _ => {
          speed_update = (0, 0);
        }
      }
      fn check_collision_with_trees(player: &Rect, tree: &Rect) -> bool {
        if player.top() > tree.bottom()
          || player.bottom() < tree.top()
          || player.right() < tree.left()
          || player.left() > tree.right()
        {
          false
        } else {
          true
        }
      }
      if check_collision(&p, &static_box) {
        speed_update = (0, 0)
      }
      p.update_pos(
        speed_update,
        x_limits,
        (
          0,
          ((BG_H + (SCALE_UP * TILE_SIZE as i16) as u32) * (SCALE_UP as u32) / 2) as i32,
        ),
      );

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

      self.core.wincan.copy(&background_image, cur_bg, None)?;
      self.core.wincan.copy(p.texture(), None, player_cam_pos)?;

      self.core.wincan.present();
    }

    Ok(())
  }
}

fn main() {
  sdl_rust::runner(TITLE, SDL04::init);
}
