use super::{player::Position, Renderable, Sprite};
use sdl2::pixels::Color;

pub struct Block<'a> {
    pub sprite: Sprite,
    pub collision_fn: Box<dyn Fn() -> ()>,
    animation: f32,
    has_collsion_fn: bool,
    max_animation: f32,
    texture: Result<sdl2::render::Texture<'a>, String>,
}
impl Renderable for Block<'_> {
    fn render(
        &self,
        canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
        screen_width: u32,
        screen_x: i32,
    ) -> Result<(), String> {
        let mut y = self.sprite.position.y as f32;

        if self.animation > 0.0 {
            if self.animation > self.max_animation / 2.0 {
                y -= self.max_animation - self.animation;
            } else {
                y -= self.animation;
            }
        }
        let x = self.sprite.position.x as i32 - screen_x;

        if x + self.sprite.w < 0 || x > screen_width as i32 {
            return Ok(());
        }
        match &self.texture {
            Ok(texture) => canvas.copy(
                &texture,
                None,
                sdl2::rect::Rect::new(
                    x as i32,
                    y as i32,
                    self.sprite.w as u32,
                    self.sprite.h as u32,
                ),
            )?,
            Err(_) => {
                canvas.set_draw_color(self.sprite.color);
                canvas.fill_rect(sdl2::rect::Rect::new(
                    x as i32,
                    y as i32,
                    self.sprite.w as u32,
                    self.sprite.h as u32,
                ))?;
            }
        };

        Ok(())
    }
}
impl Block<'_> {
    pub fn new(
        x: usize,
        y: usize,
        w: i32,
        h: i32,
        color: Color,
        collision_fn: Option<Box<dyn Fn() -> ()>>,
        animation: f32,
        texture: Result<sdl2::render::Texture<'_>, String>,
    ) -> Block {
        let mut has_collsion_fn = false;
        let collision_fn = match collision_fn {
            Some(f) => {
                has_collsion_fn = true;
                f
            }
            None => Box::new(|| {}),
        };
        Block {
            sprite: Sprite {
                position: Position {
                    x: x as f32,
                    y: y as f32,
                },
                color,
                w,
                h,
            },
            collision_fn,
            animation: 0.0,
            has_collsion_fn,
            max_animation: animation,
            texture,
        }
    }

    pub fn update(&mut self) {
        if self.animation > 0.0 {
            self.animation -= 1.0;
        }
    }

    pub fn collision(&mut self) {
        if self.has_collsion_fn && self.animation == 0.0{
            self.animation = self.max_animation;
            (self.collision_fn)();
        }
    }
}
