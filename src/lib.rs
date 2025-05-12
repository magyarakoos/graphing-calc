use config::{CONFIG, PARSER};
use graph::graph_function;
use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, Document};

mod config;
mod coordinate;
mod graph;
mod parser;
mod shunting_yard;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

fn get_document() -> Document {
    web_sys::window().unwrap().document().unwrap()
}

fn get_canvas() -> web_sys::HtmlCanvasElement {
    let canvas = get_document().get_element_by_id("canvas").unwrap();
    canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap()
}

fn get_context() -> CanvasRenderingContext2d {
    get_canvas()
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap()
}

#[wasm_bindgen]
pub fn render_canvas(formula: &str) {
    let canvas = get_canvas();
    let ctx = get_context();
    ctx.clear_rect(0.0, 0.0, canvas.width() as f64, canvas.height() as f64);
    match PARSER.parse(formula) {
        Ok(f) => {
            log(&format!("Formula {} rendered successfully.", formula));
            graph_function(f, &ctx)
        }
        Err(e) => log(&format!("The formula \"{}\" failed: {}", formula, e)),
    }
}

#[wasm_bindgen(start)]
fn start() {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = get_canvas();
    let ctx = get_context();

    canvas.set_width(CONFIG.width);
    canvas.set_height(CONFIG.height);
    document.set_title(&CONFIG.title);

    ctx.set_line_width(2.0);
    ctx.set_stroke_style_str("red");
}
