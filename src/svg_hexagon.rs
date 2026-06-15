use core::f32;

use glam::Vec2;

pub fn get_hexagon_points_pointy_top(center: Vec2, radius: f32) -> String {
    let mut s = String::new();
    for i in 0..6 {
        let m = f32::consts::PI * (i as f32) / 3.0;
        let xi = center.x + (radius * m.sin());
        let yi = center.y + (radius * m.cos());
        use std::fmt::Write;
        write!(&mut s, "{xi:.2},{yi:.2} ").unwrap();
    }

    s
}

pub fn get_hexagon_points_flat_top(center: Vec2, radius: f32) -> String {
    let mut s = String::new();
    for i in 0..6 {
        let m = f32::consts::PI * (i as f32) / 3.0;
        let xi = center.x + (radius * m.cos());
        let yi = center.y + (radius * m.sin());
        use std::fmt::Write;
        write!(&mut s, "{xi:.2},{yi:.2} ").unwrap();
    }

    s
}
