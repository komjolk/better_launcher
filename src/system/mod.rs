pub mod player;
use player::{Player,CollisionType, Position};
pub(crate) struct System{
    pub player: Player,
    screen_width: u32,
    screen_height: u32,
}
impl System{
    fn check_collision(&self, position : Position, momentum : Position) -> CollisionType{
        CollisionType::None
    }

    pub fn new(screen_height :u32, screen_width: u32, speed_x: f32) -> System{


        System{
            player: Player::new(0, 0, speed_x),
            screen_width: screen_width,
            screen_height: screen_height,
        }
    }
    pub fn update(&mut self){
        let collision_type = self.check_collision(self.player.position, self.player.momentum);
        if collision_type == CollisionType::None{
            self.player.position.x += self.player.momentum.x;
            self.player.position.y += self.player.momentum.y;
        }
    }



}