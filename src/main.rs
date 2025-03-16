mod config;
mod coordinate;
mod graph;
mod parser;
mod shunting_yard;

use parser::Parser;
use parser::Token::*;

//use config::{Config, parse_config};
//use graph::graph_function;
//use raylib::prelude::*;

//fn f(x: f32) -> f32 {
//    5.0 / (x * x)
//}

fn main() {
    let parser = Parser::new("../config/parser/constants.json");

    let tokens = parser.tokenize("5 + sin(x) ");
    for token in &tokens {
        match token {
            Number(n) => print!("{n} "),
            Function(_) => print!("F "),
            X => print!("X "),
            OpenParen => print!("( "),
            CloseParen => print!(") "),
        }
    }
    println!("\n{}", tokens.len());

    //let cfg: Config = parse_config("../config.json")
    //    .map_err(|e| {
    //        panic!("Problem parsing the JSON: {e:?}");
    //    })
    //    .unwrap();
    //
    //let (mut rl, thread) = raylib::init()
    //    .size(cfg.width, cfg.height)
    //    .title(&cfg.title)
    //    .build();
    //
    //rl.set_target_fps(cfg.fps);
    //
    //while !rl.window_should_close() {
    //    let mut d = rl.begin_drawing(&thread);
    //    d.clear_background(Color::WHITE);
    //    graph_function(f, &mut d, &cfg);
    //}
}
