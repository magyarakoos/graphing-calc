use crate::config::Config;

pub fn interpolate(value: f32, min: f32, max: f32, new_min: f32, new_max: f32) -> Option<f32> {
    if value < min || value > max || min > max || new_min > new_max {
        None
    } else {
        Some((value - min) / (max - min) * (new_max - new_min) + new_min)
    }
}

pub fn pixel_to_coordinate(x: i32, y: i32, cfg: &Config) -> Option<(f32, f32)> {
    let x = interpolate(
        (x - cfg.width / 2) as f32,
        -(cfg.width / 2) as f32,
        (cfg.width / 2) as f32,
        cfg.min_x,
        cfg.max_x,
    )?;
    let y = interpolate(
        (y - cfg.height / 2) as f32,
        -(cfg.height / 2) as f32,
        (cfg.height / 2) as f32,
        cfg.min_y,
        cfg.max_y,
    )?;
    Some((x, y))
}

pub fn coordinate_to_pixel(x: f32, y: f32, cfg: &Config) -> Option<(i32, i32)> {
    let x = cfg.width
        - (interpolate(
            x,
            cfg.min_x,
            cfg.max_x,
            -(cfg.width / 2) as f32,
            (cfg.width / 2) as f32,
        )?
        .round() as i32
            + cfg.width / 2);
    let y = cfg.height
        - (interpolate(
            y,
            cfg.min_y,
            cfg.max_y,
            -(cfg.height / 2) as f32,
            (cfg.height / 2) as f32,
        )?
        .round() as i32
            + cfg.height / 2);
    Some((x, y))
}
