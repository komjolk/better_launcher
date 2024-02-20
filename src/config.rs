use std::{fs::File, io::Read};


#[derive(serde::Deserialize, Debug)]
pub struct Config {
    pub blocks : Vec<BlockConfig>,
    pub player : PlayerConfig,
    pub screen : ScreenConfig
}
#[derive(serde::Deserialize, Debug, Clone)]
pub struct BlockConfig {
    pub x: usize,
    pub y: usize,
    pub w: i32,
    pub h: i32,
    pub command: Vec<String>,
    pub color : [u8;3]
}
#[derive(serde::Deserialize, Debug)]
pub struct PlayerConfig {
    pub x: usize,
    pub y: usize,
    pub w: i32,
    pub h: i32,
    pub speed: f32,
    pub gravity: f32,
    pub jump_speed: f32,
    pub color : [u8;3],
    pub friction: f32
}
#[derive(serde::Deserialize, Debug)]
pub struct ScreenConfig {
    pub w: u32,
    pub h: u32,
    pub color : [u8;3]
}

pub fn read_config(path: &str) -> Result<Config, String> {
        // Open the file in read mode
        let mut file = File::open(path).expect("File not found");
    
        // Read the file content into a String
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("Failed to read file");
    
        // Deserialize the JSON data into your structure
        let data: Config = serde_json::from_str(&contents).expect("Failed to parse JSON");
    
        // Now 'data' contains the deserialized JSON data
        println!("{:?}", data);
        Ok(data)
}