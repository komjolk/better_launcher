use sdl2::{event::Event, keyboard::Keycode};

use super::player::{Direction, Player};

#[derive(Debug)]
pub struct Keys {
    pub w: bool,
    pub s: bool,
    pub a: bool,
    pub d: bool,
    pub right_arrow: bool,
    pub left_arrow: bool,
    pub up_arrow: bool,
    pub down_arrow: bool,
    pub space: bool,
}


impl Keys {
    pub fn new() -> Keys {
        Keys {
            w: false,
            a: false,
            s: false,
            d: false,
            right_arrow : false,
            left_arrow : false,
            up_arrow : false,
            down_arrow : false,
            space: false,
        }
    }
    pub fn update(&mut self, event_pump: &mut sdl2::EventPump, player: &mut Player) -> Result<(), String>{
    for event in event_pump.poll_iter() {
        match event {
            Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => return Err("Quit".to_string()),
            Event::KeyDown {
                keycode: Some(Keycode::W),
                ..
            } => {
                self.w = true;
            }
            Event::KeyUp {
                keycode: Some(Keycode::W),
                ..
            } => {
                self.w = false;
            }
            Event::KeyDown {
                keycode: Some(Keycode::S),
                ..
            } => {
                self.s = true;
            }
            Event::KeyUp {
                keycode: Some(Keycode::S),
                ..
            } => {
                self.s = false;
            }
            Event::KeyDown {
                keycode: Some(Keycode::A),
                ..
            } => {
                self.a = true;
            }
            Event::KeyUp {
                keycode: Some(Keycode::A),
                ..
            } => {
                self.a = false;
            }
            Event::KeyDown {
                keycode: Some(Keycode::D),
                ..
            } => {
                self.d = true;
            }
            Event::KeyUp {
                keycode: Some(Keycode::D),
                ..
            } => {
                self.d = false;
            }
            Event::KeyDown {
                keycode: Some(Keycode::Space),
                ..
            } => {
                self.space = true;
            }
            Event::KeyUp {
                keycode: Some(Keycode::Space),
                ..
            } => {
                self.space = false;
            }
            Event::KeyDown {
                keycode: Some(Keycode::Right),
                ..
            } => {
                self.right_arrow = true;
            }
            Event::KeyUp {
                keycode: Some(Keycode::Right),
                ..
            } => {
                self.right_arrow = false;
            }
            Event::KeyDown {
                keycode: Some(Keycode::Left),
                ..
            } => {
                self.left_arrow = true;
            }
            Event::KeyUp {
                keycode: Some(Keycode::Left),
                ..
            } => {
                self.left_arrow = false;
            }
            Event::KeyDown {
                keycode: Some(Keycode::Up),
                ..
            } => {
                self.up_arrow = true;
            } 
            Event::KeyUp {
                keycode: Some(Keycode::Up),
                ..
            } => {
                self.up_arrow = false;
            }


            _ => {}
        }

        
    }
    if self.w || self.up_arrow ||self.space{
        player.move_player(Direction::Up);
    }
    if self.s || self.down_arrow{
        player.move_player(Direction::Down);
    }
    if self.a || self.left_arrow{
        player.move_player(Direction::Left);
    }
    if self.d || self.right_arrow{
        player.move_player(Direction::Right);
    }
Ok(())

}
}