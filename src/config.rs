use once_cell::sync::Lazy;
use serde::Deserialize;
use std::fs;

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    pub width: i32,
    pub height: i32,
    pub fps: u32,
    pub title: String,
    pub min_x: f64,
    pub min_y: f64,
    pub max_x: f64,
    pub max_y: f64,
}

pub static CONFIG: Lazy<Config> = Lazy::new(|| {
    let data = fs::read_to_string("config/config.json").unwrap();
    let cfg: Config = serde_json::from_str(&data).expect("Invalid JSON");
    cfg
});
