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
    let num_rays = framebuffer.width;

    let _hw = framebuffer.width as f32 / 2.0; // precalculated half width
    let hh = framebuffer.height as f32 / 2.0; // precalculated half height

    for i in 0..num_rays {
        let current_ray = i as f32 / num_rays as f32; // current ray divided by total rays
        let a = player.a - (player.fov / 2.0) + (player.fov * current_ray);
        let intersect = cast_ray(framebuffer, &maze, &player, a, block_size, false);

        // Calculate the height of the stake
        let distance_to_wall = intersect.distance; // how far is this wall from the player
        let distance_to_projection_plane = 277.0; // how far is the "player" from the "camera"

        // Calculate shading based on distance
        let shade = ((255.0 / distance_to_wall).min(255.0)) as u8;
        framebuffer.set_current_color(Color::new(shade, shade, shade, 255));

        // Calculate stake height with distance compensation
        let stake_height = (hh / distance_to_wall) * distance_to_projection_plane;

        // Calculate the position to draw the stake, clamping to screen bounds
        let stake_top = ((hh - (stake_height / 2.0)) as usize).max(0);
        let stake_bottom = ((hh + (stake_height / 2.0)) as usize).min(framebuffer.height as usize);

        // Draw the stake directly in the framebuffer with shading
        for y in stake_top..stake_bottom {
            framebuffer.set_pixel(i as u32, y as u32);
        }
    }
}
