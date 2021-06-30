//use std::time::Duration;
//use std::thread;
//use std::collections::HashSet;

use std::path::Path;

use sdl2::pixels::Color;
use sdl2::image::LoadTexture;
use sdl2::render::TextureQuery;
//use sdl2::event::Event;
//use sdl2::keyboard::Keycode;

use sdl2::rect::Rect;

const CAM_W: u32 = 1280;
const CAM_H: u32 = 720;
// const TIMEOUT: u64 = 200;

fn center(r1: Rect, w: u32, h: u32) -> Rect {
    let mut x = r1.x();
    let mut y = r1.y();
    if r1.width() < w {
        x += ((w - r1.width()) / 2) as i32
    }
    if r1.height() < h {
        y += ((h - r1.height()) / 2) as i32
    }
    Rect::new(x, y, r1.width(), r1.height())

}

fn fit(r1: Rect, w: u32, h: u32) -> Rect {
    let c = if w as f32/r1.width() as f32 > h as f32/r1.height() as f32 { // wider
        r1.width() as f32 / w as f32
    } else {
        r1.height() as f32 / h as f32
    };
    Rect::new(r1.x(), r1.y(), (w as f32 *c) as u32, (h as f32 *c) as u32)
} 

pub fn load_monsters(wincan: &mut sdl2::render::WindowCanvas, player_monster: &str, enemy_monster: &str) -> Result<(), String> {
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;
    let font_path = Path::new(r".\fonts\framd.ttf");
    let font = ttf_context.load_font(font_path, 256)?;
    
    let pi = format!("images/{}.png", player_monster);
    let ei = format!("images/{}.png", enemy_monster);
    
    let texture_creator = wincan.texture_creator();
    let t1 = texture_creator.load_texture(pi)?;
    let t2 = texture_creator.load_texture(ei)?;

    wincan.copy(&t1, None, Rect::new(800,275,200,200))?;
    wincan.copy_ex(&t2, None, Rect::new(280 as i32,25 as i32,200,200), 0 as f64, None, true, false)?;

    let surface = font
        .render(&enemy_monster)
        .blended(Color::BLACK)
        .map_err(|e| e.to_string())?;
    let texture = texture_creator
        .create_texture_from_surface(&surface)
        .map_err(|e| e.to_string())?;

    let TextureQuery { width, height, .. } = texture.query();

    let text_rect = Rect::new(490, 88, 300, 35);
    let text_rect = fit(text_rect, width, height);

    wincan.copy(&texture, None, text_rect)?;

    let surface = font
        .render(&player_monster)
        .blended(Color::BLACK)
        .map_err(|e| e.to_string())?;
    let texture = texture_creator
        .create_texture_from_surface(&surface)
        .map_err(|e| e.to_string())?;

    let TextureQuery { width, height, .. } = texture.query();

    let text_rect = Rect::new(500, 379, 300, 35);
    let text_rect = fit(text_rect, width, height);
    let text_rect = Rect::new(500 + (290 - text_rect.width() as i32) as i32, text_rect.y(), text_rect.width(), text_rect.height());

    wincan.copy(&texture, None, text_rect)?;

    Ok(())

}

pub fn health_bars(wincan: &mut sdl2::render::WindowCanvas, player_health: f32, enemy_health: f32) -> Result<(), String> {
    //let enemy_health: f32 = 12 as f32;

    if enemy_health > 50 as f32{
        wincan.set_draw_color(Color::GREEN);
    } else if enemy_health > 20 as f32{
        wincan.set_draw_color(Color::YELLOW);
    } else if enemy_health == 0 as f32 {
        wincan.set_draw_color(Color::RGBA(0,0,0,0));
    } else {
        wincan.set_draw_color(Color::RED);
    }

    let r2 = Rect::new(508, 54, ((enemy_health*435.0/100.0) as f32).ceil() as u32, 18);
    wincan.fill_rect(r2)?;

    // let r2 = Rect::new(333, 429, 435, 18);

    // let player_health: f32 = 51 as f32;

    if player_health > 50 as f32{
        wincan.set_draw_color(Color::GREEN);
    } else if player_health > 20 as f32{
        wincan.set_draw_color(Color::YELLOW);
    } else if player_health == 0 as f32 {
        wincan.set_draw_color(Color::RGBA(0,0,0,0));
    } else {
        wincan.set_draw_color(Color::RED);
    }

    let r2 = Rect::new(333, 429, ((player_health*435.0/100.0) as f32).ceil() as u32, 18);
    wincan.fill_rect(r2)?;

    // wincan.present();

    Ok(())
}

pub fn dialogue_box(wincan: &mut sdl2::render::WindowCanvas, message: &str) -> Result<(), String> {
    let texture_creator = wincan.texture_creator();
    
    let r2 = Rect::new(600, 150, 400, 125);
    wincan.set_draw_color(Color::WHITE);
    wincan.fill_rect(r2)?;

    let r2 = Rect::new(605, 155, 390, 115);
    wincan.set_draw_color(Color::BLACK);
    wincan.fill_rect(r2)?;

    let r2 = Rect::new(610, 160, 380, 105);
    wincan.set_draw_color(Color::WHITE);
    wincan.fill_rect(r2)?;
    
    let mut st = format!("");

    let mut line_number = 0;

    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;
    // let font_path = Path::new(r".\fonts\AgencyFB-Bold.ttf");
    // let font_path = Path::new(r".\fonts\joystix monospace.ttf");
    let font_path = Path::new(r".\fonts\framd.ttf");

    // Load a font
    let font = ttf_context.load_font(font_path, 256)?;

    for c in message.chars() {
        st = format!("{}{}", st, c);
        
        let mut surface = font
            .render(&st)
            .blended(Color::BLACK)
            .map_err(|e| e.to_string())?;
        let mut texture = texture_creator
            .create_texture_from_surface(&surface)
            .map_err(|e| e.to_string())?;

        let TextureQuery { width, height, .. } = texture.query();

        // println!("{}", width);

        if width as f32/height as f32 > 375 as f32/30 as f32 {
            line_number += 1;
            st = format!("{}",c);
            surface = font
                .render(&st)
                .blended(Color::BLACK)
                .map_err(|e| e.to_string())?;
            texture = texture_creator
                .create_texture_from_surface(&surface)
                .map_err(|e| e.to_string())?;
        }

        let TextureQuery { width, height, .. } = texture.query();
        

        let text_rect = Rect::new(612, 162 + (line_number * 35), 375, 30);
        let text_rect = fit(text_rect, width, height);
        // let text_rect = Rect::new(500 + (290 - text_rect.width() as i32) as i32, text_rect.y(), text_rect.width(), text_rect.height());
        wincan.copy(&texture, None, text_rect)?;
        wincan.present();
    }

    Ok(())
}

pub fn draw_battle(wincan: &mut sdl2::render::WindowCanvas, choice: usize) -> Result<(), String> {
    let texture_creator = wincan.texture_creator();

    wincan.set_draw_color(Color::RGBA(0, 128, 128, 255));
    wincan.clear();

    let bg = texture_creator.load_texture("images/battle_bg.png")?;
    wincan.copy(&bg, None, Rect::new(0,0,CAM_W,CAM_H))?;

    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;
    // let font_path = Path::new(r".\fonts\AgencyFB-Bold.ttf");
    // let font_path = Path::new(r".\fonts\joystix monospace.ttf");
    let font_path = Path::new(r".\fonts\framd.ttf");

    // Load a font
    let font = ttf_context.load_font(font_path, 256)?;
    // font.set_style(sdl2::ttf::FontStyle::BOLD);

    let rs: Vec<_> = (0..4)
        .map(|i| 180 + i * (200 + 40))
        .map(|i| Rect::new(i, 560 as i32, 200, 100))
        .collect();

    let outline_size = 5;
    let r = rs[choice];
    let r2 = Rect::new(r.x() - outline_size, r.y() - outline_size, (r.width() + (2*outline_size)as u32) as u32, (r.height() + (2*outline_size)as u32) as u32);

    wincan.set_draw_color(Color::RGB(0xf6, 0x52, 0x41));
    wincan.fill_rect(r2)?;

    for (index, item) in rs.into_iter().enumerate()  {
        let r = item;
        wincan.set_draw_color(Color::RGB(0x20, 0x41, 0x6a));
        wincan.fill_rect(r)?;

        let s = format!("Attack {}", index+1);
        let surface = font
            .render(&s)
            .blended(Color::RGB(0xbd, 0xcd, 0xde))
            .map_err(|e| e.to_string())?;
        let texture = texture_creator
            .create_texture_from_surface(&surface)
            .map_err(|e| e.to_string())?;

        let TextureQuery { width, height, .. } = texture.query();
        
        let text_rect = Rect::new(r.x() + 10, r.y() + 5, 180, 50);
        let text_rect = center(fit(text_rect, width, height), 180, 50);

        wincan.copy(&texture, None, text_rect)?;

        let s = format!("Effects for {}", index+1);
        let surface = font
            .render(&s)
            .blended(Color::RGB(0xbd, 0xcd, 0xde))
            .map_err(|e| e.to_string())?;
        let texture = texture_creator
            .create_texture_from_surface(&surface)
            .map_err(|e| e.to_string())?;

        let TextureQuery { width, height, .. } = texture.query();

        let text_rect = Rect::new(r.x() + 10, r.y() + 65, 180, 30);
            
        let text_rect = center(fit(text_rect, width, height), 180, 30);

        wincan.copy(&texture, None, text_rect)?;

        

		
    }

    
    // FOR DEMO ONLY

    let s = vec!["Demo Instructions:","Use AD/←→ to choose a move","Use Enter to submit your choice", "Use K to kill the other monster", "Use E to exit the battle"];
    
    for (index, item) in s.iter().enumerate() {
        let surface = font
            .render(&item)
            .blended(Color::BLACK)
            .map_err(|e| e.to_string())?;
        let texture = texture_creator
            .create_texture_from_surface(&surface)
            .map_err(|e| e.to_string())?;

        let TextureQuery { width, height, .. } = texture.query();

        let text_rect = Rect::new(25, 300 + (20*index) as i32, width, 20);
        let text_rect = fit(text_rect, width, height);
        wincan.copy(&texture, None, text_rect)?;
    }


    Ok(())
}
