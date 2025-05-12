use web_sys::CanvasRenderingContext2d;

use crate::config::CONFIG;
use crate::coordinate::{coordinate_to_pixel, pixel_to_coordinate};

pub fn graph_function<F>(f: F, ctx: &CanvasRenderingContext2d)
where
    F: Fn(f64) -> f64,
{
    ctx.begin_path();

    let mut prev_xy: Option<(i32, i32)> = None;
    for x in 0..CONFIG.width {
        let cx = pixel_to_coordinate(x as i32, 0).0;
        let cy = f(cx);
        let y = coordinate_to_pixel(0.0, cy.clamp(-1e4, 1e4)).1;

        prev_xy = match prev_xy {
            Some((_, prev_y)) => {
                if ((prev_y - y).abs() as u32) < CONFIG.height {
                    ctx.line_to(x as f64, y as f64);
                }
                Some((x as i32, y))
            }
            None => {
                ctx.move_to(x as f64, y as f64);
                Some((x as i32, y))
            }
        }
    }

    ctx.stroke();
}
