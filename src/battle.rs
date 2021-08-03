use sdl2::pixels::Color;
use sdl2::render::TextureQuery;
use sdl2::image::LoadTexture;
use sdl2::rect::Rect;

use std::time::Duration;
use std::thread;
use std::collections::HashMap;

use rand::{self, Rng};

use crate::monster;

pub enum Map {
    Intro,
    Hospital,
    Home,
    Overworld,
    Battle,
    GymOne,
    GymTwo,
    GymThree,
    GymFour,
}

const CAM_W: u32 = 1280;
const CAM_H: u32 = 720;
const MESSAGE_TIME: u64 = 500;

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

pub fn create_all_attack_textures<'a, T>(
    texture_creator: &'a sdl2::render::TextureCreator<T>, 
    font: &'a sdl2::ttf::Font,
    attack_names: &'a Vec<String>,
) -> Result<HashMap<String, sdl2::render::Texture<'a>>, String> {

    let mut attacks_map = HashMap::new();

    for item in attack_names.into_iter() {
        let surface = font
            .render(item)
            .blended(Color::RGB(0xbd, 0xcd, 0xde))
            .map_err(|e| e.to_string())?;
        let texture = texture_creator
            .create_texture_from_surface(&surface)
            .map_err(|e| e.to_string())?;

        attacks_map.insert(item.clone(), texture);
    }

    Ok( attacks_map )
}

pub fn create_all_effect_textures<'a, T>(
    texture_creator: &'a sdl2::render::TextureCreator<T>, 
    font: &'a sdl2::ttf::Font,
    attack_effects: &'a Vec<String>,
) -> Result<HashMap<String, sdl2::render::Texture<'a>>, String> {

    let mut attacks_map = HashMap::new();

    for item in attack_effects.into_iter() {
        let surface = font
            .render(item)
            .blended(Color::RGB(0xbd, 0xcd, 0xde))
            .map_err(|e| e.to_string())?;
        let texture = texture_creator
            .create_texture_from_surface(&surface)
            .map_err(|e| e.to_string())?;

        attacks_map.insert(item.clone(), texture);
    }

    Ok( attacks_map )
}

pub fn create_all_name_tuples<'a, T>(
    texture_creator: &'a sdl2::render::TextureCreator<T>, 
    font: &'a sdl2::ttf::Font,
    monster_names: &'a Vec<String>,
) -> Result<HashMap<String, (sdl2::render::Texture<'a>, Rect, Rect)>, String> {
    
    let mut monster_name_map = HashMap::new();

    for item in monster_names.into_iter() {
        let surface = font
            .render(item)
            .blended(Color::BLACK)
            .map_err(|e| e.to_string())?;
        let player_texture = texture_creator
            .create_texture_from_surface(&surface)
            .map_err(|e| e.to_string())?;

        let TextureQuery { width, height, .. } = player_texture.query();

        let text_rect = Rect::new(500, 379, 300, 35);
        let text_rect = fit(text_rect, width, height);
        let player_rect = Rect::new(500 + (290 - text_rect.width() as i32) as i32, text_rect.y(), text_rect.width(), text_rect.height());
    
        let text_rect = Rect::new(490, 88, 300, 35);
        let enemy_rect = fit(text_rect, width, height);

        monster_name_map.insert(item.clone(), (player_texture, player_rect, enemy_rect));
    }
    Ok( monster_name_map )
}

pub fn create_all_monster_textures<'a, T>(
    texture_creator: &'a sdl2::render::TextureCreator<T>, 
    monster_names: &'a Vec<String>,
) -> Result<HashMap<String, sdl2::render::Texture<'a>>, String> {
    let mut monster_text_map = HashMap::new();
    
    for item in monster_names.into_iter() {
        let im_path = format!("images/{}.png", item);
        let temp_text = texture_creator.load_texture(im_path)?;
        monster_text_map.insert(item.clone(), temp_text);
    }

    Ok( monster_text_map )
}

pub struct Battle<'a> {
    pub background_texture: &'a sdl2::render::Texture<'a>,
    pub player_name: String,
    pub enemy_name: String,
    pub font: &'a sdl2::ttf::Font<'a, 'a>,
    pub player_health: f32,
    pub enemy_health: f32,
    pub name_text_map: &'a HashMap<String, (sdl2::render::Texture<'a>, Rect, Rect)>,
    pub attack_map: &'a HashMap<String, sdl2::render::Texture<'a>>,
    pub effect_map: &'a HashMap<String, sdl2::render::Texture<'a>>,
    pub monster_text_map: &'a HashMap<String, sdl2::render::Texture<'a>>,
    pub moves: &'a HashMap<String, monster::Move>,
    pub monsters: &'a HashMap<String, monster::Monster<'a>>,
}

impl<'a> Battle<'a> {
    pub fn apply_player_damage(&mut self, damage: f32) {
        self.player_health -= damage;
        self.player_health = self.player_health.clamp(0.0, 100.0);
    }
    pub fn apply_enemy_damage(&mut self, damage: f32) {
        self.enemy_health -= damage;
        self.enemy_health = self.enemy_health.clamp(0.0, 100.0);
    }
}

pub fn draw_battle(
    wincan: &mut sdl2::render::WindowCanvas,
    battle_init: &Battle,
    choice: Option<usize>,
    message: Option<String>,
) -> Result<(), String> {
    // Load the battle scene background
    wincan.copy(&battle_init.background_texture, None, Rect::new(0,0,CAM_W,CAM_H))?;

    let move_rects: Vec<_> = (0..4)
        .map(|i| 180 + i * (200 + 40))
        .map(|i| Rect::new(i, 560 as i32, 200, 100))
        .collect();

    // Create an outline around the move that is currently selected
    let outline_size = 5;

    match choice {
        Some(option) => {
            let r = move_rects[option];
            let move_outline_rect = Rect::new(
                r.x() - outline_size,
                r.y() - outline_size,
                (r.width() + (2 * outline_size) as u32) as u32,
                (r.height() + (2 * outline_size) as u32) as u32,
            );

            wincan.set_draw_color(Color::RGB(0xf6, 0x52, 0x41));
            wincan.fill_rect(move_outline_rect)?;
        }
        None => {}
    };

    // For all moves
    for (index, item) in move_rects.into_iter().enumerate() {
        // Create the background for each move
        let r = item;
        wincan.set_draw_color(Color::RGB(0x20, 0x41, 0x6a));
        wincan.fill_rect(r)?;

        let attack_name = &battle_init.monsters[&battle_init.player_name].moves[index].name;
        let texture = &battle_init.attack_map[attack_name];
        
        // Add the names of each attack
        // Figure out how to resize the text to fit within the provided space
        let TextureQuery { width, height, .. } = texture.query();
        let text_rect = Rect::new(r.x() + 10, r.y() + 5, 180, 50);
        let text_rect = center(fit(text_rect, width, height), 180, 50);
        
        wincan.copy(&texture, None, text_rect)?;
        
        let effect_name = &battle_init.monsters[&battle_init.player_name].moves[index].effect;
        let texture = &battle_init.effect_map[effect_name];
        
        // Add the names of each effect
        let TextureQuery { width, height, .. } = texture.query();
        let text_rect = Rect::new(r.x() + 10, r.y() + 65, 180, 30);
        let text_rect = center(fit(text_rect, width, height), 180, 30);
        
        wincan.copy(&texture, None, text_rect)?;
    }

    // Add the names of both monsters
    wincan.copy(&battle_init.name_text_map[&battle_init.player_name].0, None, battle_init.name_text_map[&battle_init.player_name].1)?;
    wincan.copy(&battle_init.name_text_map[&battle_init.enemy_name].0, None, battle_init.name_text_map[&battle_init.enemy_name].2)?;

    // Add both monsters
    wincan.copy(&battle_init.monster_text_map[&battle_init.player_name], None, Rect::new(800,275,200,200))?;
    wincan.copy_ex(&battle_init.monster_text_map[&battle_init.enemy_name], None, Rect::new(280 as i32,25 as i32,200,200), 0 as f64, None, true, false)?;

    // Calculate and add health bars for each monster
    health_bars(wincan, battle_init.player_health, battle_init.enemy_health)?;
    
    // Print out a message if needed
    match message {
        Some(text) => {
            message_box(wincan, battle_init.font, &text)?;
            thread::sleep(Duration::from_millis(MESSAGE_TIME));
        }
        None => (),
    };

    wincan.present();

    Ok(())
}

pub fn health_bars(wincan: &mut sdl2::render::WindowCanvas, player_health: f32, enemy_health: f32) -> Result<(), String> {
    
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

    Ok(())
}

fn message_box<'a>(
    wincan: &mut sdl2::render::WindowCanvas, 
    font: &'a sdl2::ttf::Font,
    message: &str,
) -> Result<(), String> {
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

    let mut line_message = format!("");

    let mut line_number = 0;

    for c in message.chars() {
        line_message = format!("{}{}", line_message, c);
        
        let mut surface = font
            .render(&line_message)
            .blended(Color::BLACK)
            .map_err(|e| e.to_string())?;
        let mut texture = texture_creator
            .create_texture_from_surface(&surface)
            .map_err(|e| e.to_string())?;

        let TextureQuery { width, height, .. } = texture.query();

        if width as f32/height as f32 > 375 as f32/30 as f32 {
            line_number += 1;
            line_message = format!("{}", c);
            surface = font
                .render(&line_message)
                .blended(Color::BLACK)
                .map_err(|e| e.to_string())?;
            texture = texture_creator
                .create_texture_from_surface(&surface)
                .map_err(|e| e.to_string())?;
        }

        let TextureQuery { width, height, .. } = texture.query();
        
        let text_rect = Rect::new(612, 162 + (line_number * 35), 375, 30);
        let text_rect = fit(text_rect, width, height);
        
        wincan.copy(&texture, None, text_rect)?;
        wincan.present();
    }
    Ok(())
}

pub fn player_battle_turn(
    wincan: &mut sdl2::render::WindowCanvas,
    battle_state: &mut monster::BattleState,
    battle_draw: &mut Battle,
    monsters_map: &HashMap<String, monster::Monster>,
    current_choice: usize,
) -> Result<Map, String> {
    let enemy_monster = battle_draw.enemy_name.clone();
    let player_monster = battle_draw.player_name.clone();

    // Message for what move was used
    thread::sleep(Duration::from_millis(100));

    let f = format!(
        "{} used {}!",
        &player_monster, monsters_map[&player_monster].moves[current_choice].name
    );
    draw_battle(wincan, &battle_draw, None, Some(f))?;

    // Apply the damage internally and to the drawing
    let d = monster::calculate_damage(battle_draw.monsters, battle_state, current_choice, true);
    battle_draw.apply_enemy_damage(d);
    battle_state.enemy_team[0].1 = battle_draw.enemy_health;

    // Check effectiveness, and message based upon it
    let effectiveness = monster::str_effectiveness(
        &monsters_map[&player_monster].moves[current_choice],
        &monsters_map[&enemy_monster].monster_type,
    );
    match effectiveness {
        Some(s) => {
            thread::sleep(Duration::from_millis(300));
            draw_battle(wincan, &battle_draw, None, Some(s))?;
        }
        None => {
            draw_battle(wincan, &battle_draw, None, None)?;
        }
    }

    thread::sleep(Duration::from_millis(300));

    if battle_draw.enemy_health == 0.0 {
        // Write message that enemy is KO'd
        thread::sleep(Duration::from_millis(200));
        let f = format!("{} KO'd {}!", &player_monster, &enemy_monster);
        draw_battle(wincan, &battle_draw, None, Some(f))?;

        if battle_state.enemy_team.len() > 1 && battle_state.enemy_team[1].1 > 0.0 {
            battle_state.enemy_team = verify_team(&battle_state.enemy_team);
            battle_draw.enemy_health = battle_state.enemy_team[0].1;
            battle_draw.enemy_name = battle_state.enemy_team[0].0.clone();
            
            thread::sleep(Duration::from_millis(200));
            let f = format!("Enemy sent out {}!", battle_state.enemy_team[0].0);
            draw_battle(wincan, &battle_draw, None, Some(f))?;
            thread::sleep(Duration::from_millis(200));
            battle_state.player_turn = !battle_state.player_turn;
        } else {
            thread::sleep(Duration::from_millis(200));
            let f = format!("You defeated the enemy!");
            draw_battle(wincan, &battle_draw, None, Some(f))?;

            // Fade out back to the overworld
            let screen = Rect::new(0, 0, CAM_W, CAM_H);
            wincan.set_draw_color(Color::RGBA(0, 0, 0, 15));
            for _i in 0..50 {
                wincan.fill_rect(screen)?;
                wincan.present();
            }
            return Ok(Map::Overworld);
        }
    }
    battle_state.player_turn = !battle_state.player_turn;
    Ok(Map::Battle)
}

pub fn enemy_battle_turn(
    wincan: &mut sdl2::render::WindowCanvas,
    battle_state: &mut monster::BattleState,
    battle_draw: &mut Battle,
    monsters_map: &HashMap<String, monster::Monster>,
) -> Result<Map, String> {
    let enemy_monster = battle_draw.enemy_name.clone();
    let player_monster = battle_draw.player_name.clone();

    let enemy_choice = rand::thread_rng().gen_range(0..4) as usize;

    // Message for what move was used

    thread::sleep(Duration::from_millis(300));

    let f = format!(
        "{} used {}!",
        &enemy_monster, monsters_map[&enemy_monster].moves[enemy_choice].name
    );
    draw_battle(wincan, &battle_draw, None, Some(f))?;

    // Apply the damage internally and to the drawing
    let d = monster::calculate_damage(battle_draw.monsters, battle_state, enemy_choice, false);
    battle_draw.apply_player_damage(d);
    battle_state.player_team[0].1 = battle_draw.player_health;
    
    // Check effectiveness, and message based upon it
    let effectiveness = monster::str_effectiveness(
        &monsters_map[&enemy_monster].moves[enemy_choice],
        &monsters_map[&player_monster].monster_type,
    );
    match effectiveness {
        Some(s) => {
            thread::sleep(Duration::from_millis(300));
            draw_battle(wincan, &battle_draw, None, Some(s))?;
        }
        None => {
            draw_battle(wincan, &battle_draw, None, None)?;
        }
    }

    thread::sleep(Duration::from_millis(300));

    if battle_draw.player_health == 0.0 {
        // Write message that player is KO'd
        thread::sleep(Duration::from_millis(200));
        let f = format!("{} KO'd {}!", &enemy_monster, &player_monster);
        draw_battle(wincan, &battle_draw, None, Some(f))?;
        
        if battle_state.player_team.len() > 1 && battle_state.player_team[1].1 > 0.0 {
            battle_state.player_team = verify_team(&battle_state.player_team);
            battle_draw.player_health = battle_state.player_team[0].1;
            battle_draw.player_name = battle_state.player_team[0].0.clone();
            
            thread::sleep(Duration::from_millis(200));
            let f = format!("Player sent out {}!", battle_state.player_team[0].0);
            draw_battle(wincan, &battle_draw, None, Some(f))?;
            thread::sleep(Duration::from_millis(200));
            battle_state.player_turn = !battle_state.player_turn;
        } else {
            thread::sleep(Duration::from_millis(200));
            let f = format!("You blacked out!");
            draw_battle(wincan, &battle_draw, None, Some(f))?;

            // Fade out back to the overworld
            let screen = Rect::new(0, 0, CAM_W, CAM_H);
            wincan.set_draw_color(Color::RGBA(0, 0, 0, 15));
            for _i in 0..50 {
                wincan.fill_rect(screen)?;
                wincan.present();
            }

            for item in battle_state.player_team.iter_mut() {
                (*item).1 = 100.0;
            }
            return Ok(Map::Overworld);
        }
    }
    battle_state.player_turn = !battle_state.player_turn;
    Ok(Map::Battle)
}

fn menu_health_bars(
    wincan: &mut sdl2::render::WindowCanvas,
    health: f32,
    x: i32,
    y: i32,
    w: u32,
    h: u32,
) -> Result<(), String> {
    if health > 50 as f32 {
        wincan.set_draw_color(Color::GREEN);
    } else if health > 20 as f32 {
        wincan.set_draw_color(Color::YELLOW);
    } else if health == 0 as f32 {
        wincan.set_draw_color(Color::RGBA(0, 0, 0, 0));
    } else {
        wincan.set_draw_color(Color::RED);
    }

    let r = Rect::new(x, y, w, h);
    wincan.fill_rect(r)?;
    Ok(())
}

pub fn draw_monster_menu(
    wincan: &mut sdl2::render::WindowCanvas,
    battle_init: &Battle,
    battle_state: &monster::BattleState,
    choice: usize,
    selected_choice: Option<usize>,
) -> Result<(), String> {
    let player_team = &battle_state.player_team;

    // Create menu background
    //wincan.set_draw_color(Color::RGB(0, 38, 255));
    wincan.set_draw_color(Color::RGB(0x20, 0x41, 0x6a));
    wincan.fill_rect(Rect::new(100, 80, 350, 560))?;
    wincan.fill_rect(Rect::new(470, 80, 710, 560))?;

    // Create a slot for each team member and a confirmation button
    let mut rects = Vec::new();
    for i in 1..4 {
        let j = i * 4 - 1;
        rects.push(Rect::new(506, j * 35, 301, 105));
        rects.push(Rect::new(843, (j + 1) * 35, 301, 105));
    }
    rects.push(Rect::new(750, 560, 150, 50));

    // Outline the currently selected option
    let outline_size = 5;
    let r = rects[choice];
    let move_outline_rect = Rect::new(
        r.x() - outline_size,
        r.y() - outline_size,
        (r.width() + (2 * outline_size) as u32) as u32,
        (r.height() + (2 * outline_size) as u32) as u32,
    );
    wincan.set_draw_color(Color::RGB(0xf6, 0x52, 0x41));
    wincan.fill_rect(move_outline_rect)?;

    match selected_choice {
        Some(c) => {
            // Outline the currently selected option
            let outline_size = 5;
            let r = rects[c];
            let move_outline_rect = Rect::new(
                r.x() - outline_size,
                r.y() - outline_size,
                (r.width() + (2 * outline_size) as u32) as u32,
                (r.height() + (2 * outline_size) as u32) as u32,
            );
            wincan.set_draw_color(Color::YELLOW);
            wincan.fill_rect(move_outline_rect)?;
        }
        None => {}
    }

    // Draw each slot
    wincan.set_draw_color(Color::RGB(0x39, 0x7B, 0xB4));
    for (_index, item) in rects.iter().enumerate() {
        wincan.fill_rect(*item)?;
    }

    // Draw OK button
    let texture_creator = wincan.texture_creator();
    let f = String::from("OK");
    let surface = battle_init
        .font
        .render(&f)
        .blended(Color::BLACK)
        .map_err(|e| e.to_string())?;
    let texture = texture_creator
        .create_texture_from_surface(&surface)
        .map_err(|e| e.to_string())?;
    let TextureQuery { width, height, .. } = texture.query();
    let text_rect = Rect::new(750, 560, 150, 50);
    let text_rect = center(fit(text_rect, width, height), 150, 50);
    wincan.copy(&texture, None, text_rect)?;

    // Draw monster image background
    let s = 20;
    wincan.set_draw_color(Color::RGB(0x8b, 0xa4, 0xb4)); //de, 0xee, 0xff));
    wincan.set_draw_color(Color::RGB(0x39, 0x7B, 0xB4));
    wincan.fill_rect(Rect::new(110, 90, 330, 330))?;

    if choice == 6 {
        let texture = texture_creator.load_texture("images/walk1_32.png")?;
        wincan.copy(
            &texture,
            None,
            Rect::new(100 + s, 80 + s, 350 - 2 * s as u32, 350 - 2 * s as u32),
        )?;

        let f = String::from("Player");
        let surface = battle_init
            .font
            .render(&f)
            .blended(Color::RGB(0xbd, 0xcd, 0xde))
            .map_err(|e| e.to_string())?;
        let texture = texture_creator
            .create_texture_from_surface(&surface)
            .map_err(|e| e.to_string())?;
        let TextureQuery { width, height, .. } = texture.query();
        let text_rect = Rect::new(110, 417, 330, 50);
        let text_rect = center(fit(text_rect, width, height), 330, 50);
        wincan.copy(&texture, None, text_rect)?;

        // Add stats

        // TODO: Have a count of the number of badges
        let f = format!("Badges: {}", 0);
        let surface = battle_init
            .font
            .render(&f)
            .blended(Color::RGB(0xbd, 0xcd, 0xde))
            .map_err(|e| e.to_string())?;
        let texture = texture_creator
            .create_texture_from_surface(&surface)
            .map_err(|e| e.to_string())?;
        let TextureQuery { width, height, .. } = texture.query();
        let text_rect = Rect::new(110, 470, 330, 35);
        let text_rect = center(fit(text_rect, width, height), 330, 35);
        wincan.copy(&texture, None, text_rect)?;
    } else {
        // Draw focused monster image
        wincan.copy(
            &battle_init.monster_text_map[&player_team[choice].0],
            None,
            Rect::new(100 + s, 80 + s, 350 - 2 * s as u32, 350 - 2 * s as u32),
        )?;
        let surface = battle_init
            .font
            .render(&player_team[choice].0)
            .blended(Color::RGB(0xbd, 0xcd, 0xde))
            .map_err(|e| e.to_string())?;
        let texture = texture_creator
            .create_texture_from_surface(&surface)
            .map_err(|e| e.to_string())?;
        let TextureQuery { width, height, .. } = texture.query();
        let text_rect = Rect::new(110, 417, 330, 50);
        let text_rect = center(fit(text_rect, width, height), 330, 50);
        wincan.copy(&texture, None, text_rect)?;

        // Add stats
        let f = format!(
            "Attack: {} | Defense: {}",
            &battle_init.monsters[&player_team[choice].0].attack_stat,
            &battle_init.monsters[&player_team[choice].0].defense_stat
        );
        let surface = battle_init
            .font
            .render(&f)
            .blended(Color::RGB(0xbd, 0xcd, 0xde))
            .map_err(|e| e.to_string())?;
        let texture = texture_creator
            .create_texture_from_surface(&surface)
            .map_err(|e| e.to_string())?;
        let TextureQuery { width, height, .. } = texture.query();
        let text_rect = Rect::new(110, 470, 330, 35);
        let text_rect = center(fit(text_rect, width, height), 330, 35);
        wincan.copy(&texture, None, text_rect)?;

        // Add each move
        for i in 0..4 {
            let attack_name = &battle_init.monsters[&player_team[choice].0].moves[i].name;
            let texture = &battle_init.attack_map[attack_name];

            // Add the names of each attack
            // Figure out how to resize the text to fit within the provided space
            let TextureQuery { width, height, .. } = texture.query();
            let text_rect = Rect::new(110, 485 + (30 * (i + 1)) as i32, 330, 30);
            let text_rect = center(fit(text_rect, width, height), 330, 30);

            wincan.copy(&texture, None, text_rect)?;
        }

        // Add a line to separate monster name from stats
        wincan.set_draw_color(Color::RGB(0xbd, 0xcd, 0xde));
        wincan.fill_rect(Rect::new(110, 468, 330, 2))?;
    }

    for index in 0..6 {
        let item = rects[index];
        if index < player_team.len() {
            let health = player_team[index].1;
            let name_texture = &battle_init.name_text_map[&player_team[index].0].0;
            let TextureQuery { width, height, .. } = name_texture.query();
            let text_rect = Rect::new(item.x + 5, item.y + 5, item.width() - 10, 40);
            let text_rect = center(fit(text_rect, width, height), 290, 40);

            wincan.copy(name_texture, None, text_rect)?;

            wincan.set_draw_color(Color::BLACK);
            wincan.fill_rect(Rect::new(item.x + 10, item.y + 105 - 10 - 25, 280, 25))?;
            if health > 0.0 {
                menu_health_bars(
                    wincan,
                    health,
                    item.x + 10,
                    item.y + 105 - 10 - 25,
                    ((health * 280.0 / 100.0) as f32).ceil() as u32,
                    25,
                )?;
            } else {
                let f = String::from("FAINTED");
                let surface = battle_init
                    .font
                    .render(&f)
                    .blended(Color::WHITE)
                    .map_err(|e| e.to_string())?;
                let texture = texture_creator
                    .create_texture_from_surface(&surface)
                    .map_err(|e| e.to_string())?;
                let TextureQuery { width, height, .. } = texture.query();
                let text_rect = Rect::new(item.x + 10, item.y + 105 - 10 - 25, 280, 25);
                let text_rect = center(fit(text_rect, width, height), 280, 25);
                wincan.copy(&texture, None, text_rect)?;
            }
        } else {
            wincan.set_draw_color(Color::BLACK);
            wincan.fill_rect(item)?;
        }
    }

    wincan.present();
    Ok(())
}

pub fn verify_team(v: &Vec<(String, f32)>) -> Vec<(String, f32)>{
    let mut alive : Vec<(String, f32)> = Vec::new();
    let mut dead : Vec<(String, f32)> = Vec::new();
    for item in v.iter() {
      if item.1 > 0.0 {
        alive.push(item.clone());
      }
      else {
        dead.push(item.clone());
      }
    }
    alive.append(&mut dead);
    return alive;
}

pub fn turn_calc<'a>(monsters: &HashMap<String, monster::Monster>, battle_state: &monster::BattleState) -> bool {
    return monsters[&battle_state.player_team[0].name].attack_stat >= monsters[&battle_state.enemy_team[0].0].attack_stat;
}