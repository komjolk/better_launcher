pub mod player;
use player::{CollisionType, Player, Position};
use sdl2::{pixels::Color, render::Canvas, video::Window};
mod block;
use block::Block;

pub(crate) struct System {
    pub player: Player,
    screen_width: u32,
    screen_height: u32,
    pub blocks: Vec<Block>,
    canvas: sdl2::render::Canvas<sdl2::video::Window>,
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
    fn check_collision(&self, sprite: Sprite, momentum: Position) -> CollisionType {
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
        for block in &self.blocks {
            if new_pos.x + sprite.w as f32 > block.sprite.position.x
                && new_pos.x < block.sprite.position.x + block.sprite.w as f32
            {
                if sprite.position.y + sprite.h as f32 > block.sprite.position.y
                    && sprite.position.y < block.sprite.position.y + block.sprite.h as f32
                {
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
                    if momentum.y > 0.0 {
                        new_pos.y = block.sprite.position.y - sprite.h as f32;
                    } else if momentum.y < 0.0 {
                        new_pos.y = block.sprite.position.y + block.sprite.h as f32;
                    }
                }
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

    pub fn new(
        screen_height: u32,
        screen_width: u32,
        speed_x: f32,
        canvas: Canvas<Window>,
    ) -> System {
        System {
            player: Player::new(0, 0, speed_x, 0.1, 1.0),
            screen_width,
            screen_height,
            blocks: vec![Block::new(0, 100, 50, 50), Block::new(150, 150, 50, 50)],
            canvas,
        }
    }
    pub fn update(&mut self) {
        self.canvas.clear();
        self.player.gravity();
        let collision_type = self.check_collision(self.player.sprite, self.player.momentum);
        match collision_type {
            CollisionType::Solid(pos) => {
                if self.player.sprite.position.x + self.player.momentum.x != pos.x {
                    self.player.momentum.x = 0.0;
                }
                if self.player.sprite.position.y + self.player.momentum.y != pos.y {
                    self.player.momentum.y = 0.0;
                }
                self.player.sprite.position = pos;
            }
            CollisionType::None => {
                self.player.sprite.position.x += self.player.momentum.x;
                self.player.sprite.position.y += self.player.momentum.y;
            }
        }
        self.player.render(&mut self.canvas).expect("RENDER_ERR");
        for block in &self.blocks {
            block.render(&mut self.canvas).expect("RENDER_ERR");
        }
        self.canvas.set_draw_color(Color::RGB(255, 0, 0));
        self.canvas.present();
    }
}
