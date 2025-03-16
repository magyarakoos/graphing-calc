use serde::Deserialize;
use serde_json::{Result, Value};
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

pub fn parse_config(path: &str) -> Result<Config> {
    let data = fs::read_to_string(path).map_err(|e| {
        panic!("Problem opening the file: {e:?}");
    })?;
    let cfg: Config = serde_json::from_str(&data)?;
    Ok(cfg)
}

pub fn read_json(path: &str) -> Result<Value> {
    let data = fs::read_to_string(path).map_err(|e| {
        panic!("Problem opening the file: {e:?}");
    })?;
    Ok(serde_json::from_str(&data)?)
}
