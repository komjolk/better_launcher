use std::{env, process::Command};
extern crate sdl2;

use sdl2::image::LoadTexture;
use std::time::Duration;
use system::rgb_to_color;
mod config;
mod system;
use crate::config::read_config;

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
    'running: loop {
        match system.update(&mut event_pump) {
            Ok(()) => (),
            Err(e) => {
                println!("{}", e);
                break 'running;
            }
        };
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
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
