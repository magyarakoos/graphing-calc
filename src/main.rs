mod config;
mod coordinate;
mod graph;
mod parser;
mod shunting_yard;

use graph::graph_function;
use parser::Parser;

use config::CONFIG;
use raylib::prelude::*;
use std::io::stdin;

fn main() {
    let parser = Parser::new("config/functions.json", "config/constants.json").unwrap();

    let mut line = String::new();
    let _ = stdin().read_line(&mut line).unwrap();

    let f = parser.parse(&line).unwrap();

    let (mut rl, thread) = raylib::init()
        .size(CONFIG.width, CONFIG.height)
        .title(&CONFIG.title)
        .build();

    rl.set_target_fps(CONFIG.fps);

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);
        d.draw_line(
            0,
            CONFIG.height / 2,
            CONFIG.width - 1,
            CONFIG.height / 2,
            Color::GRAY,
        );
        d.draw_line(
            CONFIG.width / 2,
            0,
            CONFIG.width / 2,
            CONFIG.height - 1,
            Color::GRAY,
        );
        graph_function(&f, &mut d);
    }
}
