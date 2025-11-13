// caster.rs

use raylib::color::Color;

use crate::framebuffer::Framebuffer;
use crate::maze::load_maze;
use crate::player::Player;
use crate::textures::TextureManager;

pub struct Intersect {
    pub distance: f32,
    pub impact: char,
    pub hit_x: f32,
    pub hit_y: f32,
    pub wall_side: char, // 'h' for horizontal, 'v' for vertical
}

pub fn cast_ray(
    framebuffer: &mut Framebuffer,
    maze: &Vec<Vec<char>>,
    player: &Player,
    a: f32,
    block_size: usize,
    draw_line: bool,
) -> Intersect {
    let mut d = 0.0;
    let step_size = 3.0; // Balance entre precisión y rendimiento (optimizado)

    framebuffer.set_current_color(Color::WHITESMOKE);
    
    // Precalcular valores trigonométricos
    let cos_a = a.cos();
    let sin_a = a.sin();

    loop {
        let x = player.pos.x + d * cos_a;
        let y = player.pos.y + d * sin_a;
        
        let grid_x = x as usize;
        let grid_y = y as usize;

        let i = grid_x / block_size;
        let j = grid_y / block_size;
        
        // Verificar límites antes de acceder
        if j >= maze.len() || i >= maze[0].len() {
            // Retornar un intersect con distancia máxima si se sale de los límites
            return Intersect {
                distance: d,
                impact: ' ',
                hit_x: x,
                hit_y: y,
                wall_side: 'v',
            };
        }

        let cell = maze[j][i];
        
        // Los triggers ('t', 's', 'c') son transparentes y atravesables, tratarlos como espacios vacíos
        if cell != ' ' && cell != 't' && cell != 's' && cell != 'c' {
            // Determinar qué lado de la pared fue golpeado de manera más precisa
            let cell_x = (i * block_size) as f32;
            let cell_y = (j * block_size) as f32;
            
            // Calcular la distancia a cada borde de la celda
            let dist_to_left = x - cell_x;
            let dist_to_right = (cell_x + block_size as f32) - x;
            let dist_to_top = y - cell_y;
            let dist_to_bottom = (cell_y + block_size as f32) - y;
            
            // Determinar qué lado está más cerca
            let min_dist = dist_to_left.min(dist_to_right).min(dist_to_top).min(dist_to_bottom);
            
            let wall_side = if min_dist == dist_to_left || min_dist == dist_to_right {
                'v' // pared vertical (izquierda o derecha)
            } else {
                'h' // pared horizontal (arriba o abajo)
            };

            return Intersect {
                distance: d,
                impact: cell,
                hit_x: x,
                hit_y: y,
                wall_side,
            };
        }

        if draw_line {
            framebuffer.set_pixel(grid_x as u32, grid_y as u32);
        }

        d += step_size;
    }
}

pub fn render3d(framebuffer: &mut Framebuffer, player: &Player, texture_manager: &TextureManager, maze: &Vec<Vec<char>>) -> Vec<f32> {
    let block_size = 100;

    // Optimización: reducir rayos a la mitad para mejor rendimiento (cada 2 píxeles)
    let ray_scale = 2usize;
    let num_rays = framebuffer.width as usize / ray_scale; 

    let _hw = framebuffer.width as f32 / 2.0;
    let hh = framebuffer.height as f32 / 2.0;

    // Constantes para el renderizado
    let distance_to_projection_plane = 100.0;
    let max_distance = 1000.0; // Para normalizar el sombreado
    
    // Inicializar z-buffer
    let mut z_buffer = vec![f32::INFINITY; framebuffer.width as usize];

    for i in 0..num_rays {
        let current_ray = i as f32 / num_rays as f32;
        let a = player.a - (player.fov / 2.0) + (player.fov * current_ray);
        let intersect = cast_ray(framebuffer, &maze, &player, a, block_size, false);

        // Los triggers ('t', 's', 'c') son transparentes, no renderizar nada si el rayo golpea un trigger
        // o si no hay pared (espacio vacío o fuera de límites)
        if intersect.impact == ' ' || intersect.impact == 't' || intersect.impact == 's' || intersect.impact == 'c' {
            continue;
        }

        let distance_to_wall = intersect.distance;

        // Calcula la intensidad basada en la distancia (1.0 cerca, 0.0 lejos)
        let intensity = 1.0 - (distance_to_wall / max_distance).min(1.0);

        // Calcula la altura del stake con compensación de distancia
        let stake_height = (hh / distance_to_wall) * distance_to_projection_plane;

        // Posiciones del stake con límites de pantalla
        let stake_top = ((hh - (stake_height / 2.0)) as usize).max(0);
        let stake_bottom = ((hh + (stake_height / 2.0)) as usize).min(framebuffer.height as usize);
        
        // Calcular posición X en pantalla (escalada)
        let screen_x = i * ray_scale;

        // Calcular coordenada horizontal de textura (tx)
        let texture_x = if let Some((tex_width, _)) = texture_manager.get_texture_dimensions(intersect.impact) {
            let wall_x = if intersect.wall_side == 'v' {
                // Para paredes verticales, usar la coordenada Y del impacto
                intersect.hit_y % block_size as f32
            } else {
                // Para paredes horizontales, usar la coordenada X del impacto
                intersect.hit_x % block_size as f32
            };
            
            // Asegurar que la coordenada esté en el rango [0, block_size)
            let normalized_x = if wall_x < 0.0 {
                wall_x + block_size as f32
            } else {
                wall_x
            };
            
            // Convertir a coordenada de textura
            let tex_x = (normalized_x / block_size as f32) * tex_width as f32;
            
            // Asegurar que esté dentro de los límites y sea un entero válido
            let final_tex_x = tex_x as u32;
            if final_tex_x >= tex_width {
                tex_width - 1
            } else {
                final_tex_x
            }
        } else {
            0
        };

        // Renderiza la textura verticalmente
        for y in stake_top..stake_bottom {
            // Calcula la coordenada de textura Y (ty)
            let texture_y = if let Some((_, tex_height)) = texture_manager.get_texture_dimensions(intersect.impact) {
                let relative_y = (y - stake_top) as f32 / (stake_bottom - stake_top) as f32;
                (relative_y * tex_height as f32) as u32
            } else {
                ((y - stake_top) as f32 / (stake_bottom - stake_top) as f32 * 63.0) as u32
            };
            
            // Obtiene el color de la textura
            let texture_color = texture_manager.get_pixel_color(intersect.impact, texture_x, texture_y);
            
            // Aplica la intensidad basada en la distancia
            let r = (texture_color.r as f32 * intensity) as u8;
            let g = (texture_color.g as f32 * intensity) as u8;
            let b = (texture_color.b as f32 * intensity) as u8;
            let final_color = Color::new(r, g, b, 255);
            
            framebuffer.set_current_color(final_color);
            // Dibujar en múltiples columnas para compensar la reducción de rayos
            for offset in 0..ray_scale {
                let x_pos = (screen_x + offset).min(framebuffer.width as usize - 1);
                framebuffer.set_pixel(x_pos as u32, y as u32);
            }
        }
        
        // Actualizar z-buffer para todas las columnas de este rayo
        for offset in 0..ray_scale {
            let x_pos = (screen_x + offset).min(z_buffer.len() - 1);
            z_buffer[x_pos] = distance_to_wall;
        }
    }
    
    z_buffer
}
