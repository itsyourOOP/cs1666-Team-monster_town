use sdl2::rect::Rect;
use sdl2::render::Texture;

pub struct Player<'a> {
  pos: Rect,
  texture: Texture<'a>,
}

impl<'a> Player<'a> {
  //Create a new instance of player struct
  pub fn create(pos: Rect, texture: Texture<'a>) -> Player {
    Player { pos, texture }
  }

  pub fn x(&self) -> i32 {
    self.pos.x()
  }
  pub fn y(&self) -> i32 {
    self.pos.y()
  }
  pub fn width(&self) -> u32 {
    self.pos.width()
  }
  pub fn height(&self) -> u32 {
    self.pos.height()
  }
  pub fn set_x(&mut self, x_cor: (i32)) {
    self.pos.set_x(x_cor);
  }
  pub fn set_y(&mut self, y_cor: (i32)) {
    self.pos.set_y(y_cor);
  }
  pub fn update_pos(&mut self, vel: (i32, i32), x_bounds: (i32, i32), y_bounds: (i32, i32)) {
    self
      .pos
      .set_x((self.pos.x() + vel.0).clamp(x_bounds.0, x_bounds.1));
    self
      .pos
      .set_y((self.pos.y() + vel.1).clamp(y_bounds.0, y_bounds.1));
  }

  pub fn texture(&self) -> &Texture {
    &self.texture
  }
}
