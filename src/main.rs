extern crate nalgebra_glm as glm;

use glm::Vec3;

mod framebuffer;
mod line;
mod polygon;
mod bmp;

use crate::framebuffer::Framebuffer;
use crate::line::Line;
use crate::bmp::WriteBmp;
use crate::polygon::Polygon;

fn main() {
    let mut framebuffer = Framebuffer::new(800, 600);

    framebuffer.set_background_color(0x000000);
    framebuffer.clear();
    
    let points = vec![
    Vec3::new(165.0, 380.0, 0.0),
    Vec3::new(185.0, 360.0, 0.0),
    Vec3::new(180.0, 330.0, 0.0),
    Vec3::new(207.0, 345.0, 0.0),
    Vec3::new(233.0, 330.0, 0.0),
    Vec3::new(230.0, 360.0, 0.0),
    Vec3::new(250.0, 380.0, 0.0),
    Vec3::new(220.0, 385.0, 0.0),
    Vec3::new(205.0, 410.0, 0.0),
    Vec3::new(193.0, 383.0, 0.0),
];

    // Change the color for the filled polygon
    framebuffer.set_current_color(0xFF0000);
    // Draw the filled polygon
    framebuffer.filled_polygon(&points);

    framebuffer.set_current_color(0xFFFFFF);
    // Draw the polygon outline
    framebuffer.polygon(&points);

    let _ = framebuffer.render_buffer("out.bmp");
}
