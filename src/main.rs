mod config;
mod coordinate;
mod graph;
mod parser;
mod shunting_yard;

use parser::Parser;

use config::CONFIG;
use graph::graph_function;
use raylib::prelude::*;

fn main() {
    let parser = Parser::new("config/constants.json");

    let f = parser
        .parse("1 / sin(x)")
        .expect("The formula is malformed.");

    let (mut rl, thread) = raylib::init()
        .size(CONFIG.width, CONFIG.height)
        .title(&CONFIG.title)
        .build();

    rl.set_target_fps(CONFIG.fps);

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);
        graph_function(&f, &mut d);
    }
}
