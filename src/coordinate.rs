use crate::config::CONFIG;

pub fn interpolate(value: f32, min: f32, max: f32, new_min: f32, new_max: f32) -> Option<f32> {
    if value < min || value > max || min > max || new_min > new_max {
        None
    } else {
        Some((value - min) / (max - min) * (new_max - new_min) + new_min)
    }
}

pub fn pixel_to_coordinate(x: i32, y: i32) -> Option<(f32, f32)> {
    let x = interpolate(
        (x - CONFIG.width / 2) as f32,
        -(CONFIG.width / 2) as f32,
        (CONFIG.width / 2) as f32,
        CONFIG.min_x,
        CONFIG.max_x,
    )?;
    let y = interpolate(
        (y - CONFIG.height / 2) as f32,
        -(CONFIG.height / 2) as f32,
        (CONFIG.height / 2) as f32,
        CONFIG.min_y,
        CONFIG.max_y,
    )?;
    Some((x, y))
}

pub fn coordinate_to_pixel(x: f32, y: f32) -> Option<(i32, i32)> {
    let x = CONFIG.width
        - (interpolate(
            x,
            CONFIG.min_x,
            CONFIG.max_x,
            -(CONFIG.width / 2) as f32,
            (CONFIG.width / 2) as f32,
        )?
        .round() as i32
            + CONFIG.width / 2);
    let y = CONFIG.height
        - (interpolate(
            y,
            CONFIG.min_y,
            CONFIG.max_y,
            -(CONFIG.height / 2) as f32,
            (CONFIG.height / 2) as f32,
        )?
        .round() as i32
            + CONFIG.height / 2);
    Some((x, y))
}
