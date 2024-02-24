pub mod player;

use std::sync::Arc;

use player::{CollisionType, Player, Position};
use sdl2::{image::LoadTexture, pixels::Color, render::Canvas, video::Window};
mod block;
use super::launch;
use block::Block;
pub(crate) struct System {
    pub player: Player,
    screen_width: u32,
    screen_height: u32,
    pub blocks: Vec<Block>,
    canvas: sdl2::render::Canvas<sdl2::video::Window>,
    color: Color,
}

#[derive(Copy, Clone)]
pub struct Sprite {
    pub position: Position,
    pub w: i32,
    pub h: i32,
    pub color: Color,
}
pub trait Renderable {
    fn render(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) -> Result<(), String>;
}

impl System {
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

    pub fn new(config: crate::config::Config, mut canvas: Canvas<Window>) -> System {
        // Create a vector of references to strings
        let mut blocks = vec![];
        for block in config.blocks {
            let block = Block::new(
                block.x,
                block.y,
                block.w,
                block.h,
                rgb_to_color(block.color),
                Some(Box::new(move || launch(Box::new(block.command.clone())))),
            );
            blocks.push(block);
        }
        System {
            player: Player::new(
                config.player.x,
                config.player.y as usize,
                config.player.speed,
                config.player.gravity,
                config.player.jump_speed,
                rgb_to_color(config.player.color),
                config.player.friction,
                config.player.image,
            ),
            screen_width: config.screen.w,
            screen_height: config.screen.h,
            blocks,
            canvas,
            color: rgb_to_color(config.screen.color),
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

        self.player.render(&mut self.canvas).expect("RENDER_ERR");
        for block in &self.blocks {
            block.render(&mut self.canvas).expect("RENDER_ERR");
        }
        self.canvas.set_draw_color(self.color);
        self.canvas.present();
    }
}

fn rgb_to_color(color: [u8; 3]) -> Color {
    Color::RGB(color[0], color[1], color[2])
}
