pub mod player;

use player::{CollisionType, Player, Position};
use sdl2::{pixels::Color, render::Canvas, video::Window};
mod block;
pub use block::Block;
pub(crate) struct System<'a> {
    pub player: Player<'a>,
    screen_width: u32,
    screen_height: u32,
    pub blocks: Vec<Block<'a>>,
    canvas: sdl2::render::Canvas<sdl2::video::Window>,
    color: Color,
    screen_x: i32,
}

#[derive(Copy, Clone)]
pub struct Sprite {
    pub position: Position,
    pub w: i32,
    pub h: i32,
    pub color: Color,
}
pub trait Renderable {
    fn render(
        &self,
        canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
        screen_width: u32,
        screen_x: i32,
    ) -> Result<(), String>;
}

impl System<'_> {
    fn check_collision(&mut self, sprite: Sprite, momentum: Position) -> CollisionType {
        let mut new_pos = Position {
            x: sprite.position.x + momentum.x,
            y: sprite.position.y + momentum.y,
        };
        if sprite.position.x + momentum.x < 0.0 {
            new_pos.x = 0.0;
        } else if sprite.position.x + momentum.x + sprite.w as f32 > self.screen_width as f32 {
            new_pos.x = self.screen_width as f32 - sprite.w as f32;
        }
        if new_pos.y < 0.0 {
            new_pos.y = 0.0;
        } else if new_pos.y + sprite.h as f32 > self.screen_height as f32 {
            new_pos.y = self.screen_height as f32 - sprite.h as f32;
        }
        for block in self.blocks.iter_mut() {
            let mut has_collision = false;
            if new_pos.x + sprite.w as f32 > block.sprite.position.x
                && new_pos.x < block.sprite.position.x + block.sprite.w as f32
            {
                if sprite.position.y + sprite.h as f32 > block.sprite.position.y
                    && sprite.position.y < block.sprite.position.y + block.sprite.h as f32
                {
                    has_collision = true;
                    if momentum.x > 0.0 {
                        new_pos.x = block.sprite.position.x - sprite.w as f32;
                    } else if momentum.x < 0.0 {
                        new_pos.x = block.sprite.position.x + block.sprite.w as f32;
                    }
                }
            }
            if new_pos.y + sprite.h as f32 > block.sprite.position.y
                && new_pos.y < block.sprite.position.y + block.sprite.h as f32
            {
                if sprite.position.x + sprite.w as f32 > block.sprite.position.x
                    && sprite.position.x < block.sprite.position.x + block.sprite.w as f32
                {
                    has_collision = true;
                    if momentum.y > 0.0 {
                        new_pos.y = block.sprite.position.y - sprite.h as f32;
                    } else if momentum.y < 0.0 {
                        new_pos.y = block.sprite.position.y + block.sprite.h as f32;
                    }
                }
            }
            // could preduce a wrong collision if player hits the block from the side while going up
            if has_collision
                && sprite.position.y > block.sprite.position.y + block.sprite.w as f32
                && block.sprite.position.y + block.sprite.w as f32 > sprite.position.y + momentum.y
            {
                block.collision();
            }
        }
        if new_pos.x != sprite.position.x + momentum.x
            || new_pos.y != sprite.position.y + momentum.y
        {
            return CollisionType::Solid(Position {
                x: new_pos.x,
                y: new_pos.y,
            });
        }
        CollisionType::None
    }

    pub fn new<'a>(
        config: crate::config::Config,
        canvas: Canvas<Window>,
        texture: Result<sdl2::render::Texture<'a>, String>,
        blocks: Vec<Block<'a>>,
    ) -> System<'a> {
        // Create a vector of references to strings

        System {
            player: Player::new(
                config.player.x,
                config.player.y as usize,
                config.player.speed,
                config.player.gravity,
                config.player.jump_speed,
                rgb_to_color(config.player.color),
                config.player.friction,
                texture,
            ),
            screen_width: config.screen.w,
            screen_height: config.screen.h,
            blocks,
            canvas,
            color: rgb_to_color(config.screen.color),
            screen_x: 0,
        }
    }
    pub fn update(&mut self) {
        self.canvas.clear();
        self.player.gravity();
        let collision_type = self.check_collision(self.player.sprite, self.player.momentum);
        self.player.collision(collision_type);
        for block in self.blocks.iter_mut() {
            block.update();
        }
        println!(
            "{}, {}, {}",
            self.player.sprite.position.x,
            self.screen_width / 2,
            self.screen_x
        );
        if self.player.sprite.position.x as i32 > self.screen_x + self.screen_width as i32 / 2 + 50
        {
            self.screen_x =
                self.player.sprite.position.x as i32 - self.screen_width as i32 / 2 - 50;
        } else if self.screen_x + self.screen_width as i32 / 2 - 50
            > self.player.sprite.position.x as i32
        {
            self.screen_x =
                self.player.sprite.position.x as i32 - self.screen_width as i32 / 2 + 50;
        }

        self.player
            .render(&mut self.canvas, self.screen_width, self.screen_x)
            .expect("RENDER_ERR");
        for block in &self.blocks {
            block
                .render(&mut self.canvas, self.screen_width, self.screen_x)
                .expect("RENDER_ERR");
        }
        self.canvas.set_draw_color(self.color);
        self.canvas.present();
    }
}

pub fn rgb_to_color(color: [u8; 3]) -> Color {
    Color::RGB(color[0], color[1], color[2])
}
