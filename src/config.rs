use once_cell::sync::Lazy;
use serde::Deserialize;
use serde_json::Value;
use std::fs;

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    pub width: i32,
    pub height: i32,
    pub fps: u32,
    pub title: String,
    pub min_x: f32,
    pub min_y: f32,
    pub max_x: f32,
    pub max_y: f32,
}

pub static CONSTANTS: Lazy<Value> = Lazy::new(|| {
    let data = fs::read_to_string("config/constants.json").unwrap();
    serde_json::from_str(&data).expect("Invalid JSON")
});

pub static CONFIG: Lazy<Config> = Lazy::new(|| {
    let data = fs::read_to_string("config/config.json").unwrap();
    let cfg: Config = serde_json::from_str(&data).expect("Invalid JSON");
    cfg
});
