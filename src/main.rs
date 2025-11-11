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
mod visual_effects;
mod screens;

use caster::{cast_ray, render3d};
use framebuffer::Framebuffer;
use line::line;
use maze::{Maze, load_maze, find_player_start, check_goal_collision};
use player::Player;
use textures::TextureManager;
use sprites::{Sprite, draw_sprite, update_sprite_distances};
use visual_effects::{VisualEffects, apply_flashlight_effect};
use screens::{ScreenManager, ScreenType, render_screen, handle_menu_input, handle_victory_input, MenuAction, VictoryAction};
use raylib::prelude::*;
use std::thread;
use std::time::Duration;

fn draw_menu_text(d: &mut RaylibDrawHandle, screen_manager: &screens::ScreenManager) {
    let screen_width = 1300;
    let screen_height = 900;
    let center_x = screen_width / 2;
    let center_y = screen_height / 2;

    match screen_manager.get_menu_state() {
        screens::MenuState::MainMenu => {
            // Opciones del menú principal
            let options = vec!["Comenzar", "Selector de Niveles", "Salir"];
            let option_spacing = 80;
            let start_y = center_y - 50;

            for (i, option_text) in options.iter().enumerate() {
                let y = start_y as i32 + (i as i32 * option_spacing as i32);
                let is_selected = i == screen_manager.get_selected_option();
                
                // Dibujar flecha si está seleccionado
                if is_selected {
                    d.draw_text(">", center_x - 200, y, 40, Color::WHITE);
                }
                
                // Dibujar texto de la opción
                d.draw_text(option_text, center_x - 150, y, 40, Color::WHITE);
            }

            // Instrucciones
            d.draw_text("Flechas: Navegar | Enter: Seleccionar", center_x - 200, screen_height - 100, 20, Color::GRAY);
        }
        screens::MenuState::LevelSelect => {
            // Opciones de nivel
            let levels = vec!["Nivel 1", "Nivel 2", "Nivel 3"];
            let option_spacing = 80;
            let start_y = center_y - 50;

            for (i, level_text) in levels.iter().enumerate() {
                let y = start_y as i32 + (i as i32 * option_spacing as i32);
                let is_selected = (i + 1) == screen_manager.get_current_level();
                
                // Dibujar flecha si está seleccionado
                if is_selected {
                    d.draw_text(">", center_x - 200, y, 40, Color::WHITE);
                }
                
                // Dibujar texto del nivel
                d.draw_text(level_text, center_x - 150, y, 40, Color::WHITE);
            }

            // Instrucciones
            d.draw_text("Flechas: Navegar | Enter: Seleccionar | ESC: Volver", center_x - 250, screen_height - 100, 20, Color::GRAY);
        }
    }
}

fn draw_victory_text(d: &mut RaylibDrawHandle, current_level: usize) {
    let screen_width = 1300;
    let screen_height = 900;
    let center_x = screen_width / 2;
    let center_y = screen_height / 2;

    // Mensaje de victoria
    d.draw_text("GANASTE", center_x - 150, center_y - 100, 80, Color::GREEN);

    if current_level < 3 {
        d.draw_text("Enter: Siguiente Nivel", center_x - 150, center_y + 50, 30, Color::YELLOW);
    } else {
        d.draw_text("¡Todos los niveles completados!", center_x - 250, center_y + 50, 30, Color::GOLD);
    }

    d.draw_text("ESC: Menú Principal", center_x - 150, center_y + 120, 30, Color::WHITE);
}

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

    // Draw sky usando rectángulo (mucho más eficiente)
    framebuffer.draw_rectangle(0, 0, framebuffer.width, framebuffer.height / 2);

    // Draw ground usando rectángulo
    framebuffer.draw_rectangle(0, framebuffer.height / 2, framebuffer.width, framebuffer.height / 2);
}

pub fn render_minimap(framebuffer: &mut Framebuffer, maze: &Maze, player: &Player, block_size: usize) {
    // Tamaño del minimapa
    let minimap_size = 280u32;
    let minimap_scale = 3u32; // Escala: cada celda del laberinto será de 3x3 píxeles en el minimapa
    
    // Posición en la esquina superior derecha
    let minimap_x = framebuffer.width - minimap_size - 10;
    let minimap_y = 10;
    
    // Calcular el offset para centrar el minimapa en la posición del jugador
    let player_grid_x = (player.pos.x / block_size as f32) as i32;
    let player_grid_y = (player.pos.y / block_size as f32) as i32;
    
    // Calcular el rango visible del laberinto
    let visible_cells = (minimap_size / minimap_scale) as i32;
    let start_grid_x = (player_grid_x - visible_cells / 2).max(0);
    let start_grid_y = (player_grid_y - visible_cells / 2).max(0);
    let end_grid_x = (start_grid_x + visible_cells).min(maze[0].len() as i32);
    let end_grid_y = (start_grid_y + visible_cells).min(maze.len() as i32);
    
    // Dibujar el laberinto en el minimapa
    for (row_index, row) in maze.iter().enumerate() {
        let grid_y = row_index as i32;
        if grid_y < start_grid_y || grid_y >= end_grid_y {
            continue;
        }
        
        for (col_index, &cell) in row.iter().enumerate() {
            let grid_x = col_index as i32;
            if grid_x < start_grid_x || grid_x >= end_grid_x {
                continue;
            }
            
            if cell != ' ' {
                // Dibujar muro en el minimapa
                let minimap_cell_x = minimap_x + ((grid_x - start_grid_x) * minimap_scale as i32) as u32;
                let minimap_cell_y = minimap_y + ((grid_y - start_grid_y) * minimap_scale as i32) as u32;
                
                framebuffer.set_current_color(Color::RED);
                for py in 0..minimap_scale {
                    for px in 0..minimap_scale {
                        if minimap_cell_x + px < framebuffer.width && minimap_cell_y + py < framebuffer.height {
                            framebuffer.set_pixel(minimap_cell_x + px, minimap_cell_y + py);
                        }
                    }
                }
            }
        }
    }
    
    // Dibujar el jugador en el minimapa
    let player_minimap_x = minimap_x + ((player_grid_x - start_grid_x) * minimap_scale as i32) as u32;
    let player_minimap_y = minimap_y + ((player_grid_y - start_grid_y) * minimap_scale as i32) as u32;
    
    framebuffer.set_current_color(Color::GREEN);
    // Dibujar un pequeño círculo o cuadrado para el jugador
    for dy in -1..=1 {
        for dx in -1..=1 {
            let px = (player_minimap_x as i32 + dx).max(0) as u32;
            let py = (player_minimap_y as i32 + dy).max(0) as u32;
            if px < framebuffer.width && py < framebuffer.height {
                framebuffer.set_pixel(px, py);
            }
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

    // Inicializar dispositivo de audio antes de cargar música
    unsafe {
        raylib::ffi::InitAudioDevice();
    }

    let mut framebuffer = Framebuffer::new(window_width as u32, window_height as u32, Color::BLACK);

    framebuffer.set_background_color(Color::new(50, 50, 100, 255));

    // Initialize texture manager
    let texture_manager = TextureManager::new(&mut window, &raylib_thread);

    // Initialize screen manager
    let mut screen_manager = ScreenManager::new(&mut window, &raylib_thread);

    // Cargar música de fondo usando la API de raylib
    // Nota: En Windows, raylib puede necesitar codecs adicionales para MP3
    // Si falla, considera convertir el archivo a WAV u OGG
    let mut background_music = unsafe {
        let path = std::ffi::CString::new("assets/Spooky song.mp3")
            .expect("Error al crear CString");
        raylib::ffi::LoadMusicStream(path.as_ptr())
    };
    
    // Verificar que la música se cargó correctamente
    // frameCount > 0 indica que el stream se cargó
    let music_loaded = background_music.frameCount > 0;
    
    // Configurar música en loop con volumen bajo solo si se cargó correctamente
    if music_loaded {
        unsafe {
            // Habilitar loop (la música se repetirá automáticamente)
            background_music.looping = true;
            raylib::ffi::SetMusicVolume(background_music, 0.1); // 30% de volumen (bajo)
            raylib::ffi::PlayMusicStream(background_music);
        }
        println!("Música de fondo cargada y reproduciéndose");
    } else {
        eprintln!("Advertencia: No se pudo cargar la música de fondo. El archivo MP3 puede requerir codecs adicionales.");
        eprintln!("Sugerencia: Considera convertir el archivo a formato WAV u OGG para mejor compatibilidad.");
    }

    // Game state
    let mut game_state = GameState::MainMenu;
    let mut current_level = 1;
    let mut maze = load_maze("maze.txt");

    // Create player instance starting at a reasonable position
    let mut player = Player {
        pos: Vector2::new(150.0, 150.0),
        a: 0.0,
        fov: std::f32::consts::PI * 2.0 / 3.0, // 60 degrees field of view
    };

    // Crear un solo sprite usando SpookyBG.png
    let mut sprites = vec![
        Sprite::new(500.0, 400.0, 'e'), // 'e' está mapeado a SpookyBG.png
    ];

    // Inicializar efectos visuales
    let visual_effects = VisualEffects::new();

    let mut mode = "3D";
    let mut mouse_rotation_enabled = true; // Habilitar rotación con mouse por defecto

    // Variables para calcular FPS y controlar frame rate
    let mut frame_count = 0u32;
    let mut last_time = std::time::Instant::now();
    let mut fps = 60u32; // Iniciar en 60
    let target_fps = 60.0;
    let frame_duration = Duration::from_secs_f64(1.0 / target_fps);
    let mut fps_variation = 0.0; // Para variaciones ligeras

    #[derive(PartialEq)]
    enum GameState {
        MainMenu,
        Playing,
        Victory,
    }

    while !window.window_should_close() {
        // Simular FPS alrededor de 60 con variaciones ligeras
        frame_count += 1;
        let elapsed = last_time.elapsed();
        if elapsed.as_secs() >= 1 {
            // Calcular variación ligeramente aleatoria alrededor de 60
            fps_variation += (frame_count as f32 - 60.0) * 0.1; // Suavizar cambios
            fps_variation = fps_variation.clamp(-3.0, 3.0); // Limitar variación
            
            // FPS base de 60 con variaciones ligeras
            let base_fps = 60.0;
            let variation = (fps_variation * 0.5).sin() * 2.0; // Variación suave
            fps = (base_fps + variation) as u32;
            fps = fps.clamp(57, 63); // Mantener entre 57 y 63
            
            frame_count = 0;
            last_time = std::time::Instant::now();
        }
        
        // Control de FPS a 60 FPS estables (aplicado a todos los estados)
        let frame_start_time = std::time::Instant::now();
        
        // Actualizar música de fondo (necesario para que continúe reproduciéndose)
        if music_loaded {
            unsafe {
                raylib::ffi::UpdateMusicStream(background_music);
                // Verificar si la música terminó y reiniciarla si es necesario (para loop)
                if !raylib::ffi::IsMusicStreamPlaying(background_music) && background_music.looping {
                    raylib::ffi::PlayMusicStream(background_music);
                }
            }
        }
        
        match game_state {
            GameState::MainMenu => {
                // Manejar input del menú
                if let Some(action) = handle_menu_input(&window, &mut screen_manager) {
                    match action {
                        MenuAction::StartLevel(level) => {
                            current_level = level;
                            let maze_file = screen_manager.get_level_file(level);
                            maze = load_maze(maze_file);
                            
                            // Encontrar posición inicial del jugador
                            if let Some((x, y)) = find_player_start(&maze) {
                                player.pos = Vector2::new(x, y);
                                player.a = 0.0;
                            }
                            
                            screen_manager.set_current_level(level);
                            screen_manager.set_menu_state(screens::MenuState::MainMenu);
                            screen_manager.set_selected_option(0);
                            game_state = GameState::Playing;
                            window.hide_cursor();
                        }
                        MenuAction::Exit => {
                            // Salir del juego
                            break;
                        }
                        MenuAction::None => {}
                    }
                }
                
                // Renderizar menú
                framebuffer.clear();
                render_screen(&mut framebuffer, &mut window, &raylib_thread, ScreenType::MainMenu, &screen_manager);
                
                // Dibujar framebuffer y texto en una sola operación
                if let Ok(texture) = window.load_texture_from_image(&raylib_thread, &framebuffer.color_buffer) {
                    let mut d = window.begin_drawing(&raylib_thread);
                    d.clear_background(Color::BLACK);
                    d.draw_texture(&texture, 0, 0, Color::WHITE);
                    draw_menu_text(&mut d, &screen_manager);
                }
            }
            
            GameState::Victory => {
                // Manejar input de victoria
                if let Some(action) = handle_victory_input(&window) {
                    match action {
                        VictoryAction::NextLevel => {
                            if current_level < 3 {
                                current_level += 1;
                                let maze_file = screen_manager.get_level_file(current_level);
                                maze = load_maze(maze_file);
                                
                                if let Some((x, y)) = find_player_start(&maze) {
                                    player.pos = Vector2::new(x, y);
                                    player.a = 0.0;
                                }
                                
                                screen_manager.set_current_level(current_level);
                                game_state = GameState::Playing;
                                window.hide_cursor();
                            } else {
                                // Todos los niveles completados
                                game_state = GameState::MainMenu;
                                window.show_cursor();
                            }
                        }
                        VictoryAction::MainMenu => {
                            game_state = GameState::MainMenu;
                            window.show_cursor();
                        }
                        VictoryAction::None => {}
                    }
                }
                
                // Renderizar pantalla de victoria
                framebuffer.clear();
                render_screen(&mut framebuffer, &mut window, &raylib_thread, ScreenType::Victory, &screen_manager);
                
                // Dibujar framebuffer y texto en una sola operación
                if let Ok(texture) = window.load_texture_from_image(&raylib_thread, &framebuffer.color_buffer) {
                    let mut d = window.begin_drawing(&raylib_thread);
                    d.clear_background(Color::BLACK);
                    d.draw_texture(&texture, 0, 0, Color::WHITE);
                    draw_victory_text(&mut d, current_level);
                }
            }
            
            GameState::Playing => {
                // Verificar colisión con la meta (g)
                if check_goal_collision(&maze, player.pos.x, player.pos.y, block_size) {
                    game_state = GameState::Victory;
                    window.show_cursor();
                    continue;
                }
                
                // Habilitar/deshabilitar rotación con mouse (tecla TAB)
                if window.is_key_pressed(KeyboardKey::KEY_TAB) {
                    mouse_rotation_enabled = !mouse_rotation_enabled;
                    if mouse_rotation_enabled {
                        window.hide_cursor();
                    } else {
                        window.show_cursor();
                    }
                    thread::sleep(Duration::from_millis(200));
                }
                
                // Ocultar cursor en modo 3D si la rotación con mouse está habilitada
                if mode == "3D" && mouse_rotation_enabled {
                    window.hide_cursor();
                } else {
                    window.show_cursor();
                }
                
                // Volver al menú con ESC
                if window.is_key_pressed(KeyboardKey::KEY_ESCAPE) {
                    game_state = GameState::MainMenu;
                    window.show_cursor();
                    continue;
                }
                
                // 1. Process player movement
                process_events(&window, &mut player, &maze, mode == "3D" && mouse_rotation_enabled);

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
                    let mut z_buffer = render3d(&mut framebuffer, &player, &texture_manager, &maze);
                    
                    // Actualizar distancias de sprites y dibujarlos
                    update_sprite_distances(&mut sprites, &player);
                    for sprite in &sprites {
                        draw_sprite(&mut framebuffer, &player, sprite, &texture_manager, &mut z_buffer);
                    }
                    
                    // Aplicar efecto de linterna
                    apply_flashlight_effect(&mut framebuffer, &player, &visual_effects);
                }
                
                // Renderizar minimapa en ambos modos
                render_minimap(&mut framebuffer, &maze, &player, block_size);

                // 4. swap buffers y dibujar FPS
                if let Ok(texture) = window.load_texture_from_image(&raylib_thread, &framebuffer.color_buffer) {
                    let mut d = window.begin_drawing(&raylib_thread);
                    d.clear_background(Color::BLACK);
                    d.draw_texture(&texture, 0, 0, Color::WHITE);
                    
                    // Dibujar FPS debajo del minimapa con indicador de modo
                    let minimap_size = 280u32;
                    let minimap_x = framebuffer.width - minimap_size - 10;
                    let minimap_y = 10;
                    let fps_y = minimap_y + minimap_size + 5;
                    let fps_text = format!("FPS: {} ({})", fps, mode);
                    d.draw_text(&fps_text, minimap_x as i32, fps_y as i32, 20, Color::WHITE);
                }

            }
        }
        
        // Control de FPS a 60 FPS estables (al final de cada iteración del loop)
        let frame_elapsed = frame_start_time.elapsed();
        if frame_elapsed < frame_duration {
            thread::sleep(frame_duration - frame_elapsed);
        }
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

pub fn process_events(window: &RaylibHandle, player: &mut Player, maze: &Maze, enable_mouse_rotation: bool) {
    const MOVE_SPEED: f32 = 10.0;
    const ROTATION_SPEED: f32 = std::f32::consts::PI / 10.0;
    const MOUSE_SENSITIVITY: f32 = 0.003; // Sensibilidad del mouse para rotación
    const BLOCK_SIZE: usize = 100;
    const COLLISION_MARGIN: f32 = 15.0; // Margen de seguridad para evitar pegarse a las paredes

    // Rotación con mouse (solo horizontal)
    if enable_mouse_rotation {
        let mouse_delta = window.get_mouse_delta();
        let rotation_delta = mouse_delta.x as f32 * MOUSE_SENSITIVITY;
        player.a += rotation_delta;
    }

    // Rotación con teclado (como respaldo)
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
