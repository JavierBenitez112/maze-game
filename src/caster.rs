// caster.rs

use raylib::color::Color;

use crate::framebuffer::Framebuffer;
use crate::maze::load_maze;
use crate::player::Player;

pub struct Intersect {
    pub distance: f32,
    pub impact: char,
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

    framebuffer.set_current_color(Color::WHITESMOKE);

    loop {
        let cos = d * a.cos();
        let sin = d * a.sin();
        let x = (player.pos.x + cos) as usize;
        let y = (player.pos.y + sin) as usize;

        let i = x / block_size;
        let j = y / block_size;

        if maze[j][i] != ' ' {
            return Intersect {
                distance: d,
                impact: maze[j][i],
            };
        }

        if draw_line {
            framebuffer.set_pixel(x as u32, y as u32);
        }

        d += 10.0;
    }
}

pub fn render3d(framebuffer: &mut Framebuffer, player: &Player) {
    let maze = load_maze("./maze.txt");
    let block_size = 100;

    let num_rays = framebuffer.width ; 

    let _hw = framebuffer.width as f32 / 2.0;
    let hh = framebuffer.height as f32 / 2.0;

    // Constantes para el renderizado
    let distance_to_projection_plane = 100.0;
    let max_distance = 1000.0; // Para normalizar el sombreado

    for i in 0..num_rays {
        let current_ray = i as f32 / num_rays as f32;
        let a = player.a - (player.fov / 2.0) + (player.fov * current_ray);
        let intersect = cast_ray(framebuffer, &maze, &player, a, block_size, false);

        let distance_to_wall = intersect.distance;

        // Calcula la intensidad basada en la distancia (1.0 cerca, 0.0 lejos)
        let intensity = 1.0 - (distance_to_wall / max_distance).min(1.0);

        // Selecciona el color basado en el tipo de pared y la distancia
        let color = match intersect.impact {
            '#' => {
                // Paredes normales en tonos rojos
                let r = (255.0 * intensity) as u8;
                let g = (100.0 * intensity) as u8;
                let b = (100.0 * intensity) as u8;
                Color::new(r, g, b, 255)
            }
            'X' => {
                // Paredes especiales en tonos azules
                let r = (100.0 * intensity) as u8;
                let g = (100.0 * intensity) as u8;
                let b = (255.0 * intensity) as u8;
                Color::new(r, g, b, 255)
            }
            _ => {
                // Otras paredes en tonos verdes
                let r = (100.0 * intensity) as u8;
                let g = (255.0 * intensity) as u8;
                let b = (100.0 * intensity) as u8;
                Color::new(r, g, b, 255)
            }
        };

        framebuffer.set_current_color(color);

        // Calcula la altura del stake con compensación de distancia
        let stake_height = (hh / distance_to_wall) * distance_to_projection_plane;

        // Posiciones del stake con límites de pantalla
        let stake_top = ((hh - (stake_height / 2.0)) as usize).max(0);
        let stake_bottom = ((hh + (stake_height / 2.0)) as usize).min(framebuffer.height as usize);

        // Dibuja el stake con el color calculado
        for y in stake_top..stake_bottom {
            framebuffer.set_pixel(i as u32, y as u32);
        }
    }
}
