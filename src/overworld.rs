//use sdl2::event::Event;
use sdl2::image::LoadTexture;
//use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
//use std::collections::HashSet;

const TILE_SIZE: u32 = 16;

const CAM_W: u32 = 1280;
const CAM_H: u32 = 720;

pub fn draw_overworld(wincan: &mut sdl2::render::WindowCanvas) -> Result<(), String>{
    let texture_creator = wincan.texture_creator();

    //Commented old town map. Remove later.
    //let background_image = texture_creator.load_texture("images/MapHolder.png")?;

    // Texture

    let tree_sheet = texture_creator.load_texture("images/tree.png")?;
    let grass_sheet = texture_creator.load_texture("images/grass_patch_32.png")?;
    let water_sheet = texture_creator.load_texture("images/water_patch_32.png")?;
    
    wincan.set_draw_color(Color::RGBA(0, 128, 128, 255));
    wincan.clear();

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

      wincan.copy(&tree_sheet, src, pos)?;

      i += 1;
    }

    // Draw upper trees
    let mut i = 0;
    while i * TILE_SIZE < CAM_W {
      let src = Rect::new(((i % 4) * TILE_SIZE) as i32, 0, TILE_SIZE, 4 * TILE_SIZE);
      let pos = Rect::new((i * TILE_SIZE) as i32, 0, TILE_SIZE, 4 * TILE_SIZE);

      wincan.copy(&tree_sheet, src, pos)?;

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

      wincan.copy(&grass_sheet, src, pos_1)?;
      wincan.copy(&grass_sheet, src, pos_2)?;
      wincan.copy(&grass_sheet, src, pos_3)?;
      wincan.copy(&grass_sheet, src, pos_4)?;

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

      wincan.copy(&water_sheet, src, pos_1)?;
      wincan.copy(&water_sheet, src, pos_2)?;
      wincan.copy(&water_sheet, src, pos_3)?;
      wincan.copy(&water_sheet, src, pos_4)?;

      i += 1;
    }

    

    Ok(())
}