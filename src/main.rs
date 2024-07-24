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

    framebuffer.set_current_color(0xFFFFFF);

    let points = vec![
        Vec3::new(100.0, 100.0, 0.0),
        Vec3::new(700.0, 100.0, 0.0),
        Vec3::new(700.0, 500.0, 0.0),
        Vec3::new(100.0, 500.0, 0.0),
    ];

    // Draw the polygon outline
    framebuffer.polygon(&points);

    // Change the color for the filled polygon
    framebuffer.set_current_color(0x0000FF);

    // Draw the filled polygon
    framebuffer.filled_polygon(&points);

    let _ = framebuffer.render_buffer("output.bmp");
}
