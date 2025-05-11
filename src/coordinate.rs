use crate::config::CONFIG;

pub fn interpolate(value: f64, min: f64, max: f64, new_min: f64, new_max: f64) -> f64 {
    (value - min) / (max - min) * (new_max - new_min) + new_min
}

pub fn pixel_to_coordinate(x: i32, y: i32) -> (f64, f64) {
    let x = interpolate(
        (x - CONFIG.width / 2) as f64,
        -(CONFIG.width / 2) as f64,
        (CONFIG.width / 2) as f64,
        CONFIG.min_x,
        CONFIG.max_x,
    );
    let y = interpolate(
        (y - CONFIG.height / 2) as f64,
        -(CONFIG.height / 2) as f64,
        (CONFIG.height / 2) as f64,
        CONFIG.min_y,
        CONFIG.max_y,
    );
    (x, y)
}

pub fn coordinate_to_pixel(x: f64, y: f64) -> (i32, i32) {
    let x = CONFIG.width
        - (interpolate(
            x,
            CONFIG.min_x,
            CONFIG.max_x,
            -(CONFIG.width / 2) as f64,
            (CONFIG.width / 2) as f64,
        )
        .round() as i32
            + CONFIG.width / 2);
    let y = CONFIG.height
        - (interpolate(
            y,
            CONFIG.min_y,
            CONFIG.max_y,
            -(CONFIG.height / 2) as f64,
            (CONFIG.height / 2) as f64,
        )
        .round() as i32
            + CONFIG.height / 2);
    (x, y)
}
