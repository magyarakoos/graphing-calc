use crate::config::CONFIG;
use crate::coordinate::{coordinate_to_pixel, pixel_to_coordinate};
use raylib::prelude::*;

pub fn graph_function<F>(f: F, d: &mut RaylibDrawHandle<'_>)
where
    F: Fn(f64) -> f64,
{
    let mut prev_xy: Option<(i32, i32)> = None;
    for x in 0..CONFIG.width {
        let cx = pixel_to_coordinate(x, 0).0;
        let cy = f(cx);
        let y = coordinate_to_pixel(0.0, cy.clamp(-1e4, 1e4)).1;

        prev_xy = match prev_xy {
            Some((prev_x, prev_y)) => {
                if (prev_y - y).abs() < CONFIG.height {
                    d.draw_line_ex(
                        Vector2::new(prev_x as f32, prev_y as f32),
                        Vector2::new(x as f32, y as f32),
                        2.5,
                        Color::RED,
                    );
                }
                Some((x, y))
            }
            None => Some((x, y)),
        }
    }
}
