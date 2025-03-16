mod config;
mod coordinate;
mod graph;
mod parser;

use parser::Parser;

//use config::{Config, parse_config};
//use graph::graph_function;
//use raylib::prelude::*;

//fn f(x: f32) -> f32 {
//    5.0 / (x * x)
//}

fn main() {
    let parser = Parser::new("../config/parser/constants.json");
    println!("{}", parser.constants["e"].as_f64().unwrap());

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
