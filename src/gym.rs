extern crate rand;

//use std::collections::HashMap;

//use sdl2::event::Event;
use sdl2::image::LoadTexture;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::collections::HashSet;
use sdl2::render::WindowCanvas;
use std::time::Duration;
use std::thread;
use crate::maze;
use maze::Maze;




const TILE_SIZE: u32 = 16;

const CAM_W: u32 = 1280;
const CAM_H: u32 = 720;

pub fn display_gym_menu(wincan: &mut WindowCanvas, player_x: i32, player_y: i32) -> Result<(), String> {
  let texture_creator = wincan.texture_creator();
  let display_gym_box = texture_creator.load_texture("images/enterbuilding.png").unwrap();

  let display_box = Rect::new(500, 200, 200 ,200);
  wincan.copy(&display_gym_box, None, display_box);

  Ok(())
}

pub fn display_exit_gym_menu(wincan: &mut WindowCanvas, player_x: i32, player_y: i32) -> Result<(), String> {
  let texture_creator = wincan.texture_creator();
  let display_gym_box = texture_creator.load_texture("images/exit_gym.png").unwrap();

  let display_box = Rect::new(500, 200, 200 ,200);
  wincan.copy(&display_gym_box, None, display_box);

  Ok(())
}



pub fn draw_gym(wincan: &mut WindowCanvas, keystate:HashSet<Keycode>, maze:Maze) ->Result<(), String>{

    let gym_screen = Rect::new((0) as i32, (0) as i32, (1280) as u32, (720) as u32);
    let texture_creator = wincan.texture_creator();
    let maze_sheet = texture_creator.load_texture("images/maze.png")?;
    wincan.set_draw_color(Color::RGBA(0, 128, 128, 255));
    wincan.fill_rect(gym_screen).unwrap();
    
   // let mut gym_maze = maze::Maze::create_random_maze(16, 9);
  let mut gym_maze = maze;
  //let mut x_tw_lw_bw = 0;
  //let mut x_rw = 65;
  let mut y1 = 0;
  let mut y2 = 44;
  let mut n = 0;
  let mut m = 0;
  for row in 0..gym_maze.maze_height {
  let mut x_tw_lw_bw = 0;
  let mut x_rw = 140;
  //let mut row = 0;
   for container in 0..gym_maze.maze[row].len() {
    
        if row == 0{
        if gym_maze.maze[row][container].top_wall == true {

          let container_to_add = Rect::new(x_tw_lw_bw, y1, 140, 5);
          
          wincan.copy(&maze_sheet, None, container_to_add)?;
        }
      }
        if gym_maze.maze[row][container].left_wall == true {

          let container_to_add = Rect::new(x_tw_lw_bw, y1, 5, 49);
          wincan.copy(&maze_sheet, None, container_to_add)?;
        }
        
        if gym_maze.maze[row][container].right_wall == true {

          let container_to_add = Rect::new(x_rw, y1, 5, 49);
          wincan.copy(&maze_sheet, None, container_to_add)?;
        }
        if gym_maze.maze[row][container].bottom_wall == true {

          let container_to_add = Rect::new(x_tw_lw_bw, y2, 140, 5);
          wincan.copy(&maze_sheet, None, container_to_add)?;
        }

        x_tw_lw_bw+=140;
        x_rw+=140;
    
    }
    y1+=44;
    y2+=44;
  }


  wincan.present();
  //wincan.clear();
  //thread::sleep(Duration::from_millis(5000));
  Ok(())

}
