use crate::config::Config;
use crate::coordinate::{coordinate_to_pixel, pixel_to_coordinate};
use raylib::prelude::*;

pub fn graph_function<F>(f: F, d: &mut RaylibDrawHandle<'_>, cfg: &Config)
where
    F: Fn(f32) -> f32,
{
    let mut prev_xy: Option<(i32, i32)> = None;
    for x in 0..cfg.width {
        let (cx, _) = match pixel_to_coordinate(x, 0, &cfg) {
            Some(coord) => coord,
            None => continue,
        };
        let cy = f(cx);
        if cy >= cfg.min_y && cy < cfg.max_y {
            let (_, y) = match coordinate_to_pixel(0.0, cy, &cfg) {
                Some(pixel) => pixel,
                None => continue,
            };
            prev_xy = match prev_xy {
                Some((prev_x, prev_y)) => {
                    d.draw_line_ex(
                        Vector2::new(prev_x as f32, prev_y as f32),
                        Vector2::new(x as f32, y as f32),
                        2.0,
                        Color::RED,
                    );
                    Some((x, y))
                }
                None => Some((x, y)),
            }
        } else {
            // don't connect points where the graph isn't continuous
            prev_xy = None;
        }
    }
}
