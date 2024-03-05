use std::{env, process::Command};
extern crate sdl2;

use sdl2::keyboard::Keycode;
use sdl2::{event::Event, image::LoadTexture};
use std::time::Duration;
use system::rgb_to_color;
mod config;
mod system;
use crate::config::read_config;
use system::player::{Direction, Keys};

pub fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return Err("No config file specified".to_string());
    }
    let config = read_config(&args[1])?;
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("better_launcher", config.screen.w, config.screen.h)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let canvas: sdl2::render::Canvas<sdl2::video::Window> =
        window.into_canvas().build().map_err(|e| e.to_string())?;

    let texture_creator = canvas.texture_creator();
    let texture: Result<sdl2::render::Texture<'_>, String> =
        texture_creator.load_texture(config.player.image.clone());
    let mut blocks = vec![];
    for block in config.blocks.clone() {
        let texture: Result<sdl2::render::Texture<'_>, String> =
            texture_creator.load_texture(block.image.clone());
        let block = system::Block::new(
            block.x,
            block.y,
            block.w,
            block.h,
            rgb_to_color(block.color),
            Some(Box::new(move || launch(Box::new(block.command.clone())))),
            texture,
        );
        blocks.push(block);
    }
    let mut system = system::System::new(config, canvas, texture, blocks);

    let mut event_pump = sdl_context.event_pump()?;
    let mut held_down_keys = Keys {
        w: false,
        a: false,
        s: false,
        d: false,
        right_arrow : false,
        left_arrow : false,
        up_arrow : false,
        down_arrow : false,
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
                    held_down_keys.w = true;
                }
                Event::KeyUp {
                    keycode: Some(Keycode::W),
                    ..
                } => {
                    held_down_keys.w = false;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::S),
                    ..
                } => {
                    held_down_keys.s = true;
                }
                Event::KeyUp {
                    keycode: Some(Keycode::S),
                    ..
                } => {
                    held_down_keys.s = false;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::A),
                    ..
                } => {
                    held_down_keys.a = true;
                }
                Event::KeyUp {
                    keycode: Some(Keycode::A),
                    ..
                } => {
                    held_down_keys.a = false;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::D),
                    ..
                } => {
                    held_down_keys.d = true;
                }
                Event::KeyUp {
                    keycode: Some(Keycode::D),
                    ..
                } => {
                    held_down_keys.d = false;
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
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } => {
                    held_down_keys.right_arrow = true;
                }
                Event::KeyUp {
                    keycode: Some(Keycode::Right),
                    ..
                } => {
                    held_down_keys.right_arrow = false;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    ..
                } => {
                    held_down_keys.left_arrow = true;
                }
                Event::KeyUp {
                    keycode: Some(Keycode::Left),
                    ..
                } => {
                    held_down_keys.left_arrow = false;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    ..
                } => {
                    held_down_keys.up_arrow = true;
                } 
                Event::KeyUp {
                    keycode: Some(Keycode::Up),
                    ..
                } => {
                    held_down_keys.up_arrow = false;
                }


                _ => {}
            }
        }

        if held_down_keys.w || held_down_keys.up_arrow ||held_down_keys.space{
            system.player.move_player(Direction::Up);
        }
        if held_down_keys.s || held_down_keys.down_arrow{
            system.player.move_player(Direction::Down);
        }
        if held_down_keys.a || held_down_keys.left_arrow{
            system.player.move_player(Direction::Left);
        }
        if held_down_keys.d || held_down_keys.right_arrow{
            system.player.move_player(Direction::Right);
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
    cmd.args(&args[1..]);
    cmd.spawn().expect("Failed to launch");
}
