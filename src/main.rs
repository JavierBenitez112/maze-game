// main.rs
#![allow(unused_imports)]
#![allow(dead_code)]

use raylib::prelude::*;
use std::f32::consts::PI;

mod framebuffer;
mod sphere;

use framebuffer::Framebuffer;
use sphere::Sphere;

pub struct Camera {
    pub eye: Vector3,  // donde esta la camara en el mundo  7, 100, 10
    pub center: Vector3,     // que mira la camara  7, 100, 5
    pub up: Vector3,     // what is up? for the camera

    pub forward: Vector3,
    pub right: Vector3,
}

impl Camera {
    pub fn new(eye: Vector3, center: Vector3, up: Vector3) -> Self {
        let mut camera = Camera {
            eye,
            center,
            up,
            forward: Vector3::zero(),
            right: Vector3::zero(),
        };

        camera.update_basis();
        camera
    }    pub fn update_basis(&mut self) {
        self.forward = (self.center - self.eye).normalized();
        self.right = self.forward.cross(self.up).normalized();
        self.up = self.right.cross(self.forward);
    }

    pub fn orbit(&mut self, yaw: f32, pitch: f32) {
        let relative_pos = self.eye - self.center;

        let radius = relative_pos.length();

        let current_yaw = relative_pos.z.atan2(relative_pos.x);
        let current_pitch = (relative_pos.y / radius).asin();

        // these are spherical coordinates
        let new_yaw = current_yaw + yaw;
        let new_pitch = (current_pitch + pitch).clamp(-1.5, 1.5);

        let pitch_cos = new_pitch.cos();
        let pitch_sin = new_pitch.sin();

        // x = r * cos(a) * cos(b)
        // y = r * sin(a)
        // z = r * cos(a) * sin (b)
        let new_relative_pos = Vector3::new(
            radius * pitch_cos * new_yaw.cos(),
            radius * pitch_sin,
            radius * pitch_cos * new_yaw.sin(),
        );

        self.eye = self.center + new_relative_pos;

        self.update_basis();
    }

    pub fn basis_change(&self, p: &Vector3) -> Vector3 {
        Vector3::new(
            p.x * self.right.x + p.y * self.up.x - p.z * self.forward.x,
            p.x * self.right.y + p.y * self.up.y - p.z * self.forward.y,
            p.x * self.right.z + p.y * self.up.z - p.z * self.forward.z,
        )
    }
}pub fn render(framebuffer: &mut Framebuffer, objects: &[Sphere], camera: &Camera) {
    let width = framebuffer.width as f32;
    let height = framebuffer.height as f32;
    let aspect_ratio = width / height;
    let fov = PI / 3.0;
    let perspective_scale = (fov * 0.5).tan();

    for y in 0..framebuffer.height {
        for x in 0..framebuffer.width {
            let screen_x = (2.0 * x as f32) / width - 1.0;
            let screen_y = -(2.0 * y as f32) / height + 1.0;

            let screen_x = screen_x * aspect_ratio * perspective_scale;
            let screen_y = screen_y * perspective_scale;

            let ray_direction = Vector3::new(screen_x, screen_y, -1.0).normalized();

            let rotated_direction = camera.basis_change(&ray_direction);

            let pixel_color = cast_ray(&camera.eye, &rotated_direction, objects);

            framebuffer.set_current_color(pixel_color);
            framebuffer.set_pixel(x, y);
        }
    }
}fn main() {
    let window_width = 1300;
    let window_height = 900;
 
    let (mut window, raylib_thread) = raylib::init()
        .size(window_width, window_height)
        .title("Raytracer Example")
        .log_level(TraceLogLevel::LOG_WARNING)
        .build();


    let mut framebuffer = Framebuffer::new(window_width  as u32, window_height as u32);

    let rubber = Material {
        diffuse: Color::new(80, 0, 0, 255),
    };

    let ivory = Material {
        diffuse: Color::new(100, 100, 80, 255),
    };

    let objects = [
        Sphere {
            center: Vector3::new(1.0, 0.0, -4.0),
            radius: 1.0,
            material: ivory,
        },
        Sphere {
            center: Vector3::new(2.0, 0.0, -5.0),
            radius: 1.0,
            material: rubber,
        },
        Sphere {
            center: Vector3::new(0.0, 0.0, 0.0),
            radius: 1.0,
            material: rubber,
        },
    ];

    let mut camera = Camera::new(
        Vector3::new(0.0, 0.0, 10.0),  // eye
        Vector3::new(0.0, 0.0, 0.0),  // center
        Vector3::new(0.0, 1.0, 0.0),  // up
    );

    let rotation_speed = PI / 100.0;

    while !window.window_should_close() {
        framebuffer.clear();

        // camera controls
        if window.is_key_down(KeyboardKey::KEY_LEFT) {
            camera.orbit(rotation_speed, 0.0);
        }
        if window.is_key_down(KeyboardKey::KEY_RIGHT) {
            camera.orbit(-rotation_speed, 0.0);
        }
        if window.is_key_down(KeyboardKey::KEY_UP) {
            camera.orbit(0.0, -rotation_speed);
        }
        if window.is_key_down(KeyboardKey::KEY_DOWN) {
            camera.orbit(0.0, rotation_speed);
        }

        render(&mut framebuffer, &objects, &camera);

        framebuffer.swap_buffers(&mut window, &raylib_thread);
    }
}