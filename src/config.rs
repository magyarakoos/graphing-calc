use crate::parser::Parser;
use once_cell::sync::Lazy;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    pub width: u32,
    pub height: u32,
    pub fps: u32,
    pub title: String,
    pub min_x: f64,
    pub min_y: f64,
    pub max_x: f64,
    pub max_y: f64,
}

const CONFIG_JSON: &str = include_str!("../config/config.json");
pub const FUNCTIONS_JSON: &str = include_str!("../config/functions.json");
pub const CONSTANTS_JSON: &str = include_str!("../config/constants.json");

pub static CONFIG: Lazy<Config> = Lazy::new(|| {
    let cfg: Config = serde_json::from_str(&CONFIG_JSON).unwrap();
    cfg
});

pub static PARSER: Lazy<Parser> = Lazy::new(|| {
    Parser::new(
        serde_json::from_str(&FUNCTIONS_JSON).unwrap(),
        serde_json::from_str(&CONSTANTS_JSON).unwrap(),
    )
    .unwrap()
});
