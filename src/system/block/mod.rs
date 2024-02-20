use sdl2::pixels::Color;

use super::{Renderable, Sprite, player::Position};

pub struct Block {
    pub sprite: Sprite,
    pub collision_fn: Box<dyn Fn() -> ()>,
    animation: f32,
    has_collsion_fn: bool,
    max_animation: f32
}
impl Renderable for Block{
    fn render(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) -> Result<(), String> {
        let mut y = self.sprite.position.y as f32;


        if self.animation > 0.0 {
            if self.animation > self.max_animation / 2.0 {
                y -= self.max_animation - self.animation;
            } else {
                y -=  self.animation;
            }
        }

        canvas.set_draw_color(self.sprite.color);
        canvas.fill_rect(sdl2::rect::Rect::new(
            self.sprite.position.x as i32,
            y as i32,
            self.sprite.w as u32,
            self.sprite.h as u32,
        ))?;
    
        Ok(())
    }
}
impl Block{
    pub fn new (x: usize, y: usize, w :i32,h:i32, color: Color, collision_fn : Option<Box<dyn Fn() -> ()>> ) -> Block{
        let mut has_collsion_fn = false; 
        let collision_fn = match collision_fn {
            Some(f) => {has_collsion_fn = true; f},
            None => Box::new(||{})
        };
        Block{
            sprite: Sprite{
                position: Position{
                    x: x as f32,
                    y: y as f32,
                },
                color,
                w,
                h
            },
            collision_fn,
            animation: 0.0,
            has_collsion_fn,
            max_animation: 50.0
        }
    }

    pub fn update(&mut self){
        if self.animation > 0.0 {
            self.animation -= 1.0;
        }
    }

    pub fn collision(&mut self){
        if self.has_collsion_fn{
            self.animation = self.max_animation;
            (self.collision_fn)();
        }
    }
    
}