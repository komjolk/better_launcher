use std::{process::Command, env};
extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
mod system;
mod config;
use system::player::{Direction, Keys, CollisionType};
use crate::config::{read_config};

pub fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return Err("No config file specified".to_string());
    }
    let config = read_config("args[1]")?;
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("better_launcher", config.screen.w, config.screen.h)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    let mut system = system::System::new(config, canvas);

    let mut event_pump = sdl_context.event_pump()?;
    let mut held_down_keys = Keys {
        up: false,
        down: false,
        left: false,
        right: false,
        space: false,
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
                Event::KeyDown {
                    keycode: Some(Keycode::Space),
                    ..
                } => {
                    held_down_keys.space = true;
                }
                Event::KeyUp {
                    keycode: Some(Keycode::Space),
                    ..
                } => {
                    held_down_keys.space = false;
                }

                _ => {}
            }
        }

        if held_down_keys.up {
            system.player.move_player(Direction::Up);
        }
        if held_down_keys.down {
            system.player.move_player(Direction::Down);
        }
        if held_down_keys.left {
            system.player.move_player(Direction::Left);
        }
        if held_down_keys.right {
            system.player.move_player(Direction::Right);
        }
        if held_down_keys.space {
            system.player.move_player(Direction::Up);
        }
        system.update();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        // The rest of the game loop goes here...
    }

    Ok(())
}
fn launch(args: Box<Vec<String>>) {
    if args.len() == 0 {
        return;
    }
    println!("Launching {:?}", args);
    let mut cmd = Command::new(&args[0]);
    cmd.args(& args[1..]);
    cmd.spawn().expect("Failed to launch");
}
