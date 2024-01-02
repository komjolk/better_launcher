use sdl2::pixels::Color;

use super::{Renderable, Sprite, player::Position};


pub struct Block {
    pub sprite: Sprite,
}
impl Renderable for Block{
    fn render(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) -> Result<(), String> {
        canvas.set_draw_color(self.sprite.color);
        canvas.fill_rect(sdl2::rect::Rect::new(
            self.sprite.position.x as i32,
            self.sprite.position.y as i32,
            self.sprite.w as u32,
            self.sprite.h as u32,
        ))?;
        Ok(())
    }
}
impl Block{
    pub fn new (x: usize, y: usize, w :i32,h:i32) -> Block{
        Block{
            sprite: Sprite{
                position: Position{
                    x: x as f32,
                    y: y as f32,
                },
                color: Color::RGB(0, 0, 255),
                w,
                h
            }
        }
    }
    
}