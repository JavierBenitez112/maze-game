use raylib::prelude::*;
use std::f32::consts::PI;

#[derive(Debug, Clone)]
pub struct VisualEffects {
    pub flashlight_enabled: bool,
    pub flashlight_intensity: f32,
    pub fog_distance: f32,
    pub anxiety_level: f32,
    pub damage_effect: f32,
    pub time: f32,
}

impl VisualEffects {
    pub fn new() -> Self {
        Self {
            flashlight_enabled: true,
            flashlight_intensity: 1.0,
            fog_distance: 300.0,
            anxiety_level: 0.0,
            damage_effect: 0.0,
            time: 0.0,
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        self.time += delta_time;
        
        // Reducir efectos gradualmente
        if self.anxiety_level > 0.0 {
            self.anxiety_level = (self.anxiety_level - delta_time * 0.5).max(0.0);
        }
        
        if self.damage_effect > 0.0 {
            self.damage_effect = (self.damage_effect - delta_time * 2.0).max(0.0);
        }
    }

    pub fn trigger_anxiety(&mut self) {
        self.anxiety_level = 1.0;
    }

    pub fn trigger_damage(&mut self) {
        self.damage_effect = 1.0;
    }

    pub fn toggle_flashlight(&mut self) {
        self.flashlight_enabled = !self.flashlight_enabled;
    }
}

pub fn apply_flashlight_effect(framebuffer: &mut crate::framebuffer::Framebuffer, _player: &crate::player::Player, effects: &VisualEffects) {
    if !effects.flashlight_enabled {
        return;
    }

    let center_x = framebuffer.width / 2;
    let center_y = framebuffer.height / 2;
    let radius = 200.0 * effects.flashlight_intensity;
    let transition_width = 30.0; // Ancho de la transición suave en el borde
    
    // Crear efecto de linterna circular con un solo círculo
    for y in 0..framebuffer.height {
        for x in 0..framebuffer.width {
            let dx = x as f32 - center_x as f32;
            let dy = y as f32 - center_y as f32;
            let distance = (dx * dx + dy * dy).sqrt();
            
            let current_color = framebuffer.get_pixel(x, y);
            let modified_color;
            
            if distance <= radius {
                // Iluminar el interior del círculo
                let brightness_factor = 1.0 + (1.0 - distance / radius) * 0.2; // Más brillo en el centro
                modified_color = Color::new(
                    (current_color.r as f32 * brightness_factor).min(255.0) as u8,
                    (current_color.g as f32 * brightness_factor).min(255.0) as u8,
                    (current_color.b as f32 * brightness_factor).min(255.0) as u8,
                    current_color.a,
                );
            } else if distance <= radius + transition_width {
                // Zona de transición suave en el borde
                let transition_factor = (distance - radius) / transition_width;
                let brightness = 1.0 - transition_factor * 0.85; // Transición suave hacia oscuro
                modified_color = Color::new(
                    (current_color.r as f32 * brightness).max(0.0) as u8,
                    (current_color.g as f32 * brightness).max(0.0) as u8,
                    (current_color.b as f32 * brightness).max(0.0) as u8,
                    current_color.a,
                );
            } else {
                // Oscurecer píxeles fuera del círculo
                let darkness_factor = 0.15; // Factor de oscurecimiento constante
                modified_color = Color::new(
                    (current_color.r as f32 * darkness_factor) as u8,
                    (current_color.g as f32 * darkness_factor) as u8,
                    (current_color.b as f32 * darkness_factor) as u8,
                    current_color.a,
                );
            }
            
            framebuffer.set_pixel_color(x, y, modified_color);
        }
    }
}

pub fn apply_fog_effect(framebuffer: &mut crate::framebuffer::Framebuffer, _effects: &VisualEffects) {
    let fog_color = Color::new(100, 100, 120, 255);
    
    // Aplicar niebla basada en la distancia (simulada por la posición Y en el framebuffer)
    for y in 0..framebuffer.height {
        let distance_factor = (y as f32 / framebuffer.height as f32).min(1.0);
        let fog_intensity = (distance_factor * 2.0).min(1.0);
        
        for x in 0..framebuffer.width {
            let current_color = framebuffer.get_pixel(x, y);
            let fogged_color = Color::new(
                (current_color.r as f32 * (1.0 - fog_intensity) + fog_color.r as f32 * fog_intensity) as u8,
                (current_color.g as f32 * (1.0 - fog_intensity) + fog_color.g as f32 * fog_intensity) as u8,
                (current_color.b as f32 * (1.0 - fog_intensity) + fog_color.b as f32 * fog_intensity) as u8,
                current_color.a,
            );
            framebuffer.set_pixel_color(x, y, fogged_color);
        }
    }
}

pub fn apply_anxiety_effect(framebuffer: &mut crate::framebuffer::Framebuffer, effects: &VisualEffects) {
    if effects.anxiety_level <= 0.0 {
        return;
    }

    let intensity = effects.anxiety_level;
    let time = effects.time;
    
    // Efecto de distorsión/parpadeo
    for y in 0..framebuffer.height {
        for x in 0..framebuffer.width {
            // Crear efecto de distorsión basado en el tiempo
            let distortion_x = (time * 10.0 + x as f32 * 0.01).sin() * intensity * 2.0;
            let distortion_y = (time * 8.0 + y as f32 * 0.01).cos() * intensity * 1.5;
            
            let new_x = ((x as f32 + distortion_x) as u32).min(framebuffer.width - 1);
            let new_y = ((y as f32 + distortion_y) as u32).min(framebuffer.height - 1);
            
            if new_x != x || new_y != y {
                let source_color = framebuffer.get_pixel(new_x, new_y);
                let current_color = framebuffer.get_pixel(x, y);
                
                // Mezclar colores para crear efecto de distorsión
                let mixed_color = Color::new(
                    ((source_color.r as f32 + current_color.r as f32) / 2.0) as u8,
                    ((source_color.g as f32 + current_color.g as f32) / 2.0) as u8,
                    ((source_color.b as f32 + current_color.b as f32) / 2.0) as u8,
                    current_color.a,
                );
                framebuffer.set_pixel_color(x, y, mixed_color);
            }
        }
    }
}

pub fn apply_damage_effect(framebuffer: &mut crate::framebuffer::Framebuffer, effects: &VisualEffects) {
    if effects.damage_effect <= 0.0 {
        return;
    }

    let intensity = effects.damage_effect;
    let time = effects.time;
    
    // Efecto de pantalla roja parpadeante
    for y in 0..framebuffer.height {
        for x in 0..framebuffer.width {
            let current_color = framebuffer.get_pixel(x, y);
            
            // Crear efecto de parpadeo rojo
            let red_intensity = (time * 20.0).sin().abs() * intensity;
            let damage_color = Color::new(
                (current_color.r as f32 + red_intensity * 100.0).min(255.0) as u8,
                (current_color.g as f32 * (1.0 - red_intensity * 0.5)).max(0.0) as u8,
                (current_color.b as f32 * (1.0 - red_intensity * 0.5)).max(0.0) as u8,
                current_color.a,
            );
            
            framebuffer.set_pixel_color(x, y, damage_color);
        }
    }
}

pub fn render_visual_effects(framebuffer: &mut crate::framebuffer::Framebuffer, player: &crate::player::Player, effects: &VisualEffects) {
    // Aplicar efectos en orden
    apply_fog_effect(framebuffer, effects);
    apply_flashlight_effect(framebuffer, player, effects);
    apply_anxiety_effect(framebuffer, effects);
    apply_damage_effect(framebuffer, effects);
}
