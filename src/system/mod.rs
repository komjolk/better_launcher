pub mod player;
use player::{Player,CollisionType, Position, Sprite};
pub(crate) struct System{
    pub player: Player,
    screen_width: u32,
    screen_height: u32,
}
impl System{
    fn check_collision(&self, sprite : Sprite, momentum : Position) -> CollisionType{
        if sprite.position.x + momentum.x < 0.0 {
            return CollisionType::Solid(0.0);
        }else if sprite.position.x + momentum.x + sprite.w as f32 > self.screen_width as f32 {
            return CollisionType::Solid(self.screen_width as f32 - sprite.w as f32);
        }
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
        let collision_type = self.check_collision(self.player.sprite, self.player.momentum);
        match collision_type{
            CollisionType::Solid(x) => {self.player.momentum.x = 0.0; self.player.sprite.position.x = x},
            CollisionType::None => self.player.sprite.position.x += self.player.momentum.x, 
        }
    }



}