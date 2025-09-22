// main.rs
#![allow(unused_imports)]
#![allow(dead_code)]

mod caster;
mod framebuffer;
mod line;
mod maze;
mod player;
mod textures;
mod sprites;

use caster::{cast_ray, render3d};
use framebuffer::Framebuffer;
use line::line;
use maze::{Maze, load_maze};
use player::Player;
use textures::TextureManager;
use sprites::{Sprite, draw_sprite, update_sprite_distances};
use raylib::prelude::*;
use std::thread;
use std::time::Duration;

fn draw_cell(framebuffer: &mut Framebuffer, xo: usize, yo: usize, block_size: usize, cell: char) {
    if cell == ' ' {
        return;
    }

    framebuffer.set_current_color(Color::RED);

    for x in xo..xo + block_size {
        for y in yo..yo + block_size {
            framebuffer.set_pixel(x as u32, y as u32);
        }
    }
}

pub fn render_maze(framebuffer: &mut Framebuffer, maze: &Maze, block_size: usize, player: &Player, _texture_manager: &TextureManager) {
    // Render 2D view
    for (row_index, row) in maze.iter().enumerate() {
        for (col_index, &cell) in row.iter().enumerate() {
            let xo = col_index * block_size;
            let yo = row_index * block_size;

            draw_cell(framebuffer, xo, yo, block_size, cell);
        }
    }

    // Draw player and FOV rays
    framebuffer.set_current_color(Color::GREEN);
    for dx in -2..=2 {
        for dy in -2..=2 {
            framebuffer.set_pixel(
                (player.pos.x + dx as f32) as u32,
                (player.pos.y + dy as f32) as u32,
            );
        }
    }

    // Cast FOV rays
    let num_rays = 100;
    for i in 0..num_rays {
        let current_ray = i as f32 / num_rays as f32;
        let a = player.a - (player.fov / 2.0) + (player.fov * current_ray);
        cast_ray(framebuffer, maze, player, a, block_size, true);
    }
}

pub fn render_world(framebuffer: &mut Framebuffer, _player: &Player) {
    framebuffer.set_current_color(Color::GRAY);

    // Draw sky
    for y in 0..framebuffer.height / 2 {
        for x in 0..framebuffer.width {
            framebuffer.set_pixel(x, y);
        }
    }

    framebuffer.set_current_color(Color::GRAY);

    // Draw ground
    for y in framebuffer.height / 2..framebuffer.height {
        for x in 0..framebuffer.width {
            framebuffer.set_pixel(x, y);
        }
    }
}

fn main() {
    let window_width = 1300;
    let window_height = 900;
    let block_size = 100;

    let (mut window, raylib_thread) = raylib::init()
        .size(window_width, window_height)
        .title("Maze Game")
        .log_level(TraceLogLevel::LOG_WARNING)
        .build();

    let mut framebuffer = Framebuffer::new(window_width as u32, window_height as u32, Color::BLACK);

    framebuffer.set_background_color(Color::new(50, 50, 100, 255));

    // Initialize texture manager
    let texture_manager = TextureManager::new(&mut window, &raylib_thread);

    // Load the maze once before the loop
    let maze = load_maze("maze.txt");

    // Create player instance starting at a reasonable position
    let mut player = Player {
        pos: Vector2::new(150.0, 150.0),
        a: 0.0,
        fov: std::f32::consts::PI * 2.0 / 3.0, // 60 degrees field of view
    };

    // Crear algunos sprites de enemigos
    let mut sprites = vec![
        Sprite::new(300.0, 150.0, 'e'),
        Sprite::new(500.0, 400.0, 'e'),
        Sprite::new(700.0, 300.0, 'e'),
        Sprite::new(400.0, 600.0, 'e'),
    ];

    let mut mode = "3D";

    while !window.window_should_close() {
        // 1. Process player movement
        process_events(&window, &mut player);

        // 2. clear framebuffer
        framebuffer.clear();

        // Check for mode switch (M key)
        if window.is_key_down(KeyboardKey::KEY_M) {
            mode = if mode == "2D" { "3D" } else { "2D" };
            // Add a small delay to prevent multiple switches
            thread::sleep(Duration::from_millis(200));
        }

        // 3. Render based on mode
        if mode == "2D" {
            render_maze(&mut framebuffer, &maze, block_size, &player, &texture_manager);
        } else {
            render_world(&mut framebuffer, &player);
            let z_buffer = render3d(&mut framebuffer, &player, &texture_manager);
            
            // Actualizar distancias de sprites y dibujarlos
            update_sprite_distances(&mut sprites, &player);
            for sprite in &sprites {
                draw_sprite(&mut framebuffer, &player, sprite, &texture_manager, &mut z_buffer.clone());
            }
        }

        // 4. swap buffers
        framebuffer.swap_buffer(&mut window, &raylib_thread);

        thread::sleep(Duration::from_millis(16));
    }
}
pub fn process_events(window: &RaylibHandle, player: &mut Player) {
    const MOVE_SPEED: f32 = 10.0;
    const ROTATION_SPEED: f32 = std::f32::consts::PI / 10.0;

    if window.is_key_down(KeyboardKey::KEY_LEFT) {
        player.a -= ROTATION_SPEED;
    }
    if window.is_key_down(KeyboardKey::KEY_RIGHT) {
        player.a += ROTATION_SPEED;
    }
    if window.is_key_down(KeyboardKey::KEY_UP) {
        player.pos.x += MOVE_SPEED * player.a.cos();
        player.pos.y += MOVE_SPEED * player.a.sin();
    }
    if window.is_key_down(KeyboardKey::KEY_DOWN) {
        player.pos.x -= MOVE_SPEED * player.a.cos();
        player.pos.y -= MOVE_SPEED * player.a.sin();
    }
}
