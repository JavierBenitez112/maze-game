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
        process_events(&window, &mut player, &maze);

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
// Función para verificar colisiones con las paredes
fn check_collision(maze: &Maze, new_x: f32, new_y: f32, block_size: usize) -> bool {
    let grid_x = (new_x / block_size as f32) as usize;
    let grid_y = (new_y / block_size as f32) as usize;
    
    // Verificar límites del laberinto
    if grid_x >= maze[0].len() || grid_y >= maze.len() {
        return true; // Colisión con límites
    }
    
    // Verificar si hay una pared en la nueva posición
    maze[grid_y][grid_x] != ' '
}

// Función para verificar colisiones con margen de seguridad
fn check_collision_with_margin(maze: &Maze, x: f32, y: f32, block_size: usize, margin: f32) -> bool {
    // Verificar múltiples puntos alrededor del jugador para evitar que se pegue a las paredes
    let points = vec![
        (x - margin, y - margin), // Esquina superior izquierda
        (x + margin, y - margin), // Esquina superior derecha
        (x - margin, y + margin), // Esquina inferior izquierda
        (x + margin, y + margin), // Esquina inferior derecha
    ];
    
    for (px, py) in points {
        if check_collision(maze, px, py, block_size) {
            return true;
        }
    }
    
    false
}

pub fn process_events(window: &RaylibHandle, player: &mut Player, maze: &Maze) {
    const MOVE_SPEED: f32 = 10.0;
    const ROTATION_SPEED: f32 = std::f32::consts::PI / 10.0;
    const BLOCK_SIZE: usize = 100;
    const COLLISION_MARGIN: f32 = 15.0; // Margen de seguridad para evitar pegarse a las paredes

    if window.is_key_down(KeyboardKey::KEY_LEFT) {
        player.a -= ROTATION_SPEED;
    }
    if window.is_key_down(KeyboardKey::KEY_RIGHT) {
        player.a += ROTATION_SPEED;
    }
    
    // Movimiento hacia adelante
    if window.is_key_down(KeyboardKey::KEY_UP) {
        let new_x = player.pos.x + MOVE_SPEED * player.a.cos();
        let new_y = player.pos.y + MOVE_SPEED * player.a.sin();
        
        // Verificar colisiones con margen de seguridad
        if !check_collision_with_margin(maze, new_x, player.pos.y, BLOCK_SIZE, COLLISION_MARGIN) {
            player.pos.x = new_x;
        }
        if !check_collision_with_margin(maze, player.pos.x, new_y, BLOCK_SIZE, COLLISION_MARGIN) {
            player.pos.y = new_y;
        }
    }
    
    // Movimiento hacia atrás
    if window.is_key_down(KeyboardKey::KEY_DOWN) {
        let new_x = player.pos.x - MOVE_SPEED * player.a.cos();
        let new_y = player.pos.y - MOVE_SPEED * player.a.sin();
        
        // Verificar colisiones con margen de seguridad
        if !check_collision_with_margin(maze, new_x, player.pos.y, BLOCK_SIZE, COLLISION_MARGIN) {
            player.pos.x = new_x;
        }
        if !check_collision_with_margin(maze, player.pos.x, new_y, BLOCK_SIZE, COLLISION_MARGIN) {
            player.pos.y = new_y;
        }
    }
}
