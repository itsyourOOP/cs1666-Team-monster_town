use sdl2::rect::Rect;
use sdl2::pixels::Color;

pub fn draw_intro( 
    wincan: &mut sdl2::render::WindowCanvas,
    choice: usize,
) -> Result<(), String> {
    let top_y : i32 = choice as i32 * 193 + 136;

    let above_rect = Rect::new(75, top_y, 1130, 5);
    let below_rect = Rect::new(75, top_y+145, 1130, 5);
    let left_rect = Rect::new(75, top_y, 5, 145);
    let right_rect = Rect::new(75+1125, top_y, 5, 145);

    wincan.set_draw_color(Color::RGB(0xf6, 0x52, 0x41));
    wincan.fill_rect(above_rect)?;
    wincan.fill_rect(below_rect)?;
    wincan.fill_rect(left_rect)?;
    wincan.fill_rect(right_rect)?;

    wincan.present();
    
    Ok(())
}