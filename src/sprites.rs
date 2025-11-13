// sprites.rs

use raylib::prelude::*;
use crate::framebuffer::Framebuffer;
use crate::player::Player;
use crate::textures::TextureManager;
use crate::maze::{Maze, check_collision_with_margin, has_line_of_sight};
use std::f32::consts::PI;

pub struct Sprite {
    pub pos: Vector2,
    pub texture_char: char,
    pub distance: f32,
    pub angle: f32,
    pub facing_angle: f32,  // Ángulo hacia donde está mirando el sprite
    pub fov: f32,            // Campo de visión del sprite (en radianes)
    pub player_detected: bool, // Si el sprite ha detectado al jugador
}

impl Sprite {
    pub fn new(x: f32, y: f32, texture_char: char) -> Self {
        Sprite {
            pos: Vector2::new(x, y),
            texture_char,
            distance: 0.0,
            angle: 0.0,
            facing_angle: 0.0, // Empezar mirando hacia la derecha
            fov: std::f32::consts::PI * 2.0 / 3.0, // 120 grados de FOV (más amplio que el jugador)
            player_detected: false,
        }
    }
    
    // Verificar si el jugador está dentro del FOV del sprite
    // Requiere el maze y block_size para verificar línea de visión
    pub fn can_see_player(&self, player_pos: Vector2, maze: &Maze, block_size: usize) -> bool {
        let dx = player_pos.x - self.pos.x;
        let dy = player_pos.y - self.pos.y;
        let distance = (dx * dx + dy * dy).sqrt();
        
        // Si está muy lejos, no puede ver
        if distance > 800.0 {
            return false;
        }
        
        // Calcular ángulo desde el sprite al jugador
        let angle_to_player = dy.atan2(dx);
        
        // Calcular diferencia angular entre la dirección del sprite y el jugador
        let mut angle_diff = angle_to_player - self.facing_angle;
        
        // Normalizar a [-PI, PI]
        while angle_diff > PI {
            angle_diff -= 2.0 * PI;
        }
        while angle_diff < -PI {
            angle_diff += 2.0 * PI;
        }
        
        // Verificar si el jugador está dentro del FOV
        if angle_diff.abs() > self.fov / 2.0 {
            return false;
        }
        
        // Verificar línea de visión (que no haya paredes entre el sprite y el jugador)
        has_line_of_sight(maze, self.pos.x, self.pos.y, player_pos.x, player_pos.y, block_size)
    }
}

// Umbral de transparencia para sprites (canal alpha)
const ALPHA_THRESHOLD: u8 = 128;

// Color que se considera transparente (magenta)
const TRANSPARENT_COLOR: Color = Color::new(255, 0, 255, 255);

pub fn draw_sprite(
    framebuffer: &mut Framebuffer,
    player: &Player,
    sprite: &Sprite,
    texture_manager: &TextureManager,
    z_buffer: &mut Vec<f32>,
) {
    // Calcular ángulo desde el jugador al sprite
    let dx = sprite.pos.x - player.pos.x;
    let dy = sprite.pos.y - player.pos.y;
    let sprite_angle = dy.atan2(dx);

    // Normalizar diferencia angular a [-PI, PI]
    let mut angle_diff = sprite_angle - player.a;
    while angle_diff > PI {
        angle_diff -= 2.0 * PI;
    }
    while angle_diff < -PI {
        angle_diff += 2.0 * PI;
    }

    // Si el sprite está fuera del FOV del jugador, no dibujar
    if angle_diff.abs() > player.fov / 2.0 {
        return;
    }

    // Distancia desde el jugador al sprite
    let distance = (dx * dx + dy * dy).sqrt();
    
    // Si el sprite está muy lejos, no dibujar
    if distance > 1000.0 {
        return;
    }

    // Calcular tamaño del sprite en pantalla (escala inversamente proporcional a la distancia)
    let sprite_size = (framebuffer.height as f32 / distance * 100.0) as usize;
    
    // Calcular posición horizontal en pantalla (centrada)
    let screen_x = (framebuffer.width as f32 / 2.0 + angle_diff / (player.fov / 2.0) * framebuffer.width as f32 / 2.0) as usize;
    
    // Calcular esquina superior izquierda del sprite en pantalla
    let start_x = screen_x.saturating_sub(sprite_size / 2);
    let start_y = (framebuffer.height as f32 / 2.0 - sprite_size as f32 / 2.0) as usize;
    
    let end_x = (start_x + sprite_size).min(framebuffer.width as usize);
    let end_y = (start_y + sprite_size).min(framebuffer.height as usize);

    // Obtener dimensiones de la textura
    let (tex_width, tex_height) = texture_manager.get_texture_dimensions(sprite.texture_char)
        .unwrap_or((128, 128));

    for x in start_x..end_x {
        for y in start_y..end_y {
            // Verificar z-buffer - solo dibujar si el sprite está más cerca que la pared
            if x < z_buffer.len() && distance < z_buffer[x] {
                // Mapear píxel de pantalla a coordenadas de textura
                let tx = ((x - start_x) * tex_width as usize / sprite_size) as u32;
                let ty = ((y - start_y) * tex_height as usize / sprite_size) as u32;

                let color = texture_manager.get_pixel_color(sprite.texture_char, tx, ty);

                 // Función para verificar si un color es transparente
                 let is_transparent = |c: Color| -> bool {
                     // Verificar canal alpha si está disponible
                     if c.a < ALPHA_THRESHOLD {
                         return true;
                     }
                    
                     // Solo considerar transparentes:
                     // 1. Negro puro (0,0,0)
                     // 2. Magenta puro (255,0,255) - color de transparencia estándar
                     // 3. Píxeles con alpha muy bajo
                     // 4. Solo negros extremos (brillo < 2)
                     (c.r == 0 && c.g == 0 && c.b == 0) || 
                     (c.r == 255 && c.g == 0 && c.b == 255) // Solo negro extremo
                 };

                // Saltar píxeles transparentes
                if !is_transparent(color) {
                    // Aplicar intensidad basada en la distancia (más suave y controlada)
                    // Reducir aún más el efecto de oscurecimiento para preservar negros
                    let intensity = (1.0 - (distance / 1000.0).min(0.4)).max(0.6);
                    
                    // Para colores muy oscuros, aplicar menos oscurecimiento
                    let brightness = (color.r as u32 + color.g as u32 + color.b as u32) / 3;
                    let final_intensity = if brightness < 5 {
                        // Para colores oscuros, usar intensidad más alta
                        intensity.max(0.8)
                    } else {
                        intensity
                    };
                    
                    // Aplicar la intensidad
                    let r = (color.r as f32 * final_intensity) as u8;
                    let g = (color.g as f32 * final_intensity) as u8;
                    let b = (color.b as f32 * final_intensity) as u8;
                    let final_color = Color::new(r, g, b, 255);
                    
                    framebuffer.set_current_color(final_color);
                    framebuffer.set_pixel(x as u32, y as u32);
                }
            }
        }
    }
}

pub fn update_sprite_distances(sprites: &mut Vec<Sprite>, player: &Player) {
    for sprite in sprites.iter_mut() {
        let dx = sprite.pos.x - player.pos.x;
        let dy = sprite.pos.y - player.pos.y;
        sprite.distance = (dx * dx + dy * dy).sqrt();
        sprite.angle = dy.atan2(dx);
    }
    
    // Ordenar sprites por distancia (más lejanos primero para correcto z-buffering)
    sprites.sort_by(|a, b| b.distance.partial_cmp(&a.distance).unwrap());
}

// Función de IA para que el sprite persiga al jugador
// Utiliza el sistema de colisiones para evitar que el sprite atraviese paredes
// El sprite solo persigue cuando detecta al jugador dentro de su FOV
pub fn update_sprite_ai(sprites: &mut Vec<Sprite>, player: &Player, maze: &Maze, block_size: usize) {
    const ENEMY_SPEED: f32 = 5.0; // Velocidad del enemigo (aumentada para mejor visibilidad)
    const ROTATION_SPEED: f32 = 0.08; // Velocidad de rotación hacia el jugador
    const COLLISION_MARGIN: f32 = 12.0; // Margen de seguridad para colisiones
    const DETECTION_HYSTERESIS: f32 = std::f32::consts::PI * 0.1; // Histeresis para evitar parpadeos
    
    for sprite in sprites.iter_mut() {
        // Calcular dirección hacia el jugador
        let dx = player.pos.x - sprite.pos.x;
        let dy = player.pos.y - sprite.pos.y;
        let distance = (dx * dx + dy * dy).sqrt();
        
        // Calcular ángulo hacia el jugador
        let angle_to_player = dy.atan2(dx);
        
        // Verificar si el sprite puede ver al jugador
        let can_see = sprite.can_see_player(player.pos, maze, block_size);
        
        // Usar histeresis: una vez detectado, mantener detección con un FOV más amplio
        let detection_fov = if sprite.player_detected {
            sprite.fov + DETECTION_HYSTERESIS
        } else {
            sprite.fov
        };
        
        // Calcular diferencia angular para detección con histeresis
        let mut angle_diff = angle_to_player - sprite.facing_angle;
        while angle_diff > PI {
            angle_diff -= 2.0 * PI;
        }
        while angle_diff < -PI {
            angle_diff += 2.0 * PI;
        }
        
        let detected = can_see || (sprite.player_detected && angle_diff.abs() <= detection_fov / 2.0);
        sprite.player_detected = detected;
        
        // Rotar hacia el jugador (siempre, pero más rápido si lo detectó)
        if distance > 10.0 {
            let rotation_rate = if detected {
                ROTATION_SPEED * 2.0 // Rotar más rápido cuando detecta al jugador
            } else {
                ROTATION_SPEED * 0.5 // Rotar más lento cuando no detecta
            };
            
            // Normalizar diferencia angular
            let mut angle_diff = angle_to_player - sprite.facing_angle;
            while angle_diff > PI {
                angle_diff -= 2.0 * PI;
            }
            while angle_diff < -PI {
                angle_diff += 2.0 * PI;
            }
            
            // Rotar hacia el jugador
            if angle_diff.abs() > 0.01 {
                if angle_diff > 0.0 {
                    sprite.facing_angle += rotation_rate.min(angle_diff);
                } else {
                    sprite.facing_angle -= rotation_rate.min(-angle_diff);
                }
                
                // Normalizar ángulo
                while sprite.facing_angle > PI {
                    sprite.facing_angle -= 2.0 * PI;
                }
                while sprite.facing_angle < -PI {
                    sprite.facing_angle += 2.0 * PI;
                }
            }
        }
        
        // Solo perseguir si detectó al jugador
        if !detected || distance < 15.0 {
            continue;
        }
        
        // Normalizar dirección
        let dir_x = if distance > 0.0 { dx / distance } else { 0.0 };
        let dir_y = if distance > 0.0 { dy / distance } else { 0.0 };
        
        // Calcular movimiento deseado (moverse en la dirección hacia el jugador)
        let move_x = dir_x * ENEMY_SPEED;
        let move_y = dir_y * ENEMY_SPEED;
        
        // Intentar moverse hacia el jugador
        let new_x = sprite.pos.x + move_x;
        let new_y = sprite.pos.y + move_y;
        
        // Verificar colisiones y aplicar movimiento
        let mut moved = false;
        
        // Primero intentar moverse en diagonal (dirección completa)
        if !check_collision_with_margin(maze, new_x, new_y, block_size, COLLISION_MARGIN) {
            sprite.pos.x = new_x;
            sprite.pos.y = new_y;
            moved = true;
        } else {
            // Si hay colisión en diagonal, intentar solo en X
            if !check_collision_with_margin(maze, new_x, sprite.pos.y, block_size, COLLISION_MARGIN) {
                sprite.pos.x = new_x;
                moved = true;
            }
            // Intentar solo en Y
            if !check_collision_with_margin(maze, sprite.pos.x, new_y, block_size, COLLISION_MARGIN) {
                sprite.pos.y = new_y;
                moved = true;
            }
        }
        
        // Si aún no se pudo mover, intentar direcciones alternativas
        // Esto ayuda a que el enemigo pueda rodear obstáculos
        if !moved {
            // Intentar moverse perpendicularmente si hay un obstáculo
            let perpendicular_dirs = vec![
                (-dir_y, dir_x),  // Perpendicular 1
                (dir_y, -dir_x),  // Perpendicular 2
            ];
            
            for (perp_x, perp_y) in perpendicular_dirs {
                let alt_x = sprite.pos.x + perp_x * ENEMY_SPEED;
                let alt_y = sprite.pos.y + perp_y * ENEMY_SPEED;
                
                if !check_collision_with_margin(maze, alt_x, alt_y, block_size, COLLISION_MARGIN) {
                    sprite.pos.x = alt_x;
                    sprite.pos.y = alt_y;
                    break;
                }
            }
        }
    }
}
