use std::process::Command;
extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::Duration;

pub fn main() -> Result<(), String> {

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let mut system = System::new( 600, 800, 1.0);

    let window = video_subsystem
        .window("better_launcher", 800, 600)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    canvas.set_draw_color(Color::RGB(255, 0, 0));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump()?;
    let mut held_down_keys = Keys{
        up: false,
        down: false,
        left: false,
        right: false,
    };


    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::W),
                    ..
                } => {

                        held_down_keys.up = true;
                }
                Event::KeyUp {
                    keycode: Some(Keycode::W),
                    ..
                } => {
                    held_down_keys.up = false;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::S),
                    ..
                } => {
                    held_down_keys.down = true;
                }
                Event::KeyUp {
                    keycode: Some(Keycode::S),
                    ..
                } => {
                    held_down_keys.down = false;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::A),
                    ..
                } => {
                    held_down_keys.left = true;
                }
                Event::KeyUp {
                    keycode: Some(Keycode::A),
                    ..
                } => {
                    held_down_keys.left = false;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::D),
                    ..
                } => {
                    held_down_keys.right = true;
                }
                Event::KeyUp {
                    keycode: Some(Keycode::D),
                    ..
                } => {
                    held_down_keys.right = false;
                }

                _ => {}
            }
        }

        if held_down_keys.up{
            system.player.move_player(Direction::Up);
        }
        if held_down_keys.down{
            system.player.move_player(Direction::Down);
        }
        if held_down_keys.left{
            system.player.move_player(Direction::Left);
        }
        if held_down_keys.right{
            system.player.move_player(Direction::Right);
        }
        canvas.clear();
        system.update();
        system.player.render(&mut canvas)?;
        canvas.set_draw_color(Color::RGB(255, 0, 0));
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
        // The rest of the game loop goes here...
    }

    Ok(())
}
#[allow(dead_code)]
fn launch(app: &str, args: &[&str]) {
    let mut cmd = Command::new(app);
    cmd.args(args);
    cmd.output().expect("Failed to launch");
}
struct Player{
   position: Position,
    momentum: Position,
    speed_x: f32,
}
#[derive(Copy, Clone)]
struct Position{
    x: f32,
    y: f32,
}
impl Player{
    fn new(x: usize, y: usize, speed_x : f32) -> Player{
        Player{
            position: Position{x: x as f32, y: y as f32},
            momentum : Position{x: 0.0, y: 0.0},
            speed_x: speed_x,

        }
    }
    fn move_player(&mut self, direction: Direction){
        match direction{
            Direction::Left => self.momentum.x -= self.speed_x,
            Direction::Right => self.momentum.x += self.speed_x,
            _ => {},
        }
    }
}
enum Direction{
    Up,
    Down,
    Left,
    Right,
}
#[derive(PartialEq)]
enum CollisionType{
    Solid,
    None
}
struct Keys{
    up: bool,
    down: bool,
    left: bool,
    right: bool,
}
impl Renderable for Player{

    fn render(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) -> Result<(), String>{
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.fill_rect(sdl2::rect::Rect::new(self.position.x as i32, self.position.y as i32, 50, 50))?;
        Ok(())
    }
}
trait Renderable{
    fn render(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) -> Result<(), String>;
}

struct System{
    player: Player,
    screen_width: u32,
    screen_height: u32,
}
impl System{
    fn check_collision(&self, position : Position, momentum : Position) -> CollisionType{
        CollisionType::None
    }

    fn new(screen_height :u32, screen_width: u32, speed_x: f32) -> System{


        System{
            player: Player::new(0, 0, speed_x),
            screen_width: screen_width,
            screen_height: screen_height,
        }
    }
    fn update(&mut self){
        let collision_type = self.check_collision(self.player.position, self.player.momentum);
        if collision_type == CollisionType::None{
            self.player.position.x += self.player.momentum.x;
            self.player.position.y += self.player.momentum.y;
        }
    }



}