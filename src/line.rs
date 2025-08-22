// Line.rs
use raylib::prelude::*;
use crate::framebuffer::Framebuffer;

pub fn line(
    framebuffer: &mut Framebuffer,
    start: Vector2,
    end: Vector2,
){
    let dx = (end.x - start.x).abs();
    let dy = (end.y - start.y).abs();
    let sx = if start.x < end.x { 1.0 } else { -1.0 };
    let sy = if start.y < end.y { 1.0 } else { -1.0 };
    let mut err = dx - dy;

    let mut x = start.x;
    let mut y = start.y;

    while x != end.x || y != end.y {
        framebuffer.set_pixel(x as u32, y as u32);
        let err2 = err * 2.0;
        if err2 > -dy {
            err -= dy;
            x += sx;
        }
        if err2 < dx {
            err += dx;
            y += sy;
        }
    }
}