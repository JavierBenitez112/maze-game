// sprites.rs

use raylib::prelude::*;
use crate::framebuffer::Framebuffer;
use crate::player::Player;
use crate::textures::TextureManager;
use std::f32::consts::PI;

pub struct Sprite {
    pub pos: Vector2,
    pub texture_char: char,
    pub distance: f32,
    pub angle: f32,
}

impl Sprite {
    pub fn new(x: f32, y: f32, texture_char: char) -> Self {
        Sprite {
            pos: Vector2::new(x, y),
            texture_char,
            distance: 0.0,
            angle: 0.0,
        }
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
