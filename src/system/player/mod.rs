use sdl2::pixels::Color;

pub struct Player {
    pub momentum: Position,
    pub sprite: Sprite,
    speed_x: f32,
}
#[derive(Copy, Clone)]
pub struct Sprite {
    pub position: Position,
    pub w: i32,
    pub h: i32,
    pub color: Color,
}
#[derive(Copy, Clone)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}
impl Player {
    pub fn new(x: usize, y: usize, speed_x: f32) -> Player {
        Player {
            sprite: Sprite {
                position: Position {
                    x: x as f32,
                    y: y as f32,
                },
                color: Color::RGB(0, 0, 0),
                w: 50,
                h: 50,
            },
            momentum: Position { x: 0.0, y: 0.0 },
            speed_x: speed_x,
        }
    }
    pub fn move_player(&mut self, direction: Direction) {
        match direction {
            Direction::Left => self.momentum.x -= self.speed_x,
            Direction::Right => self.momentum.x += self.speed_x,
            _ => {}
        }
    }
}

impl Renderable for Player {
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
pub trait Renderable {
    fn render(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) -> Result<(), String>;
}

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}
#[derive(PartialEq)]
pub enum CollisionType {
    Solid(f32),
    None,
}
pub struct Keys {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
}
