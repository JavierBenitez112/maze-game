const TRANSPARENT_COLOR: Color = Color::new(152,0,136,255);

fn draw_sprite(
    framebuffer: &mut Framebuffer,
    player: &Player,
    enemy: &Enemy,
    texture_manager: &TextureManager
) {
    let sprite_a = (enemy.pos.y - player.pos.y).atan2(enemy.pos.x - player.pos.x);
    let mut angle_diff = sprite_a - player.a;
    while angle_diff > PI {
        angle_diff -= 2.0 * PI;
    }
    while angle_diff < -PI {
        angle_diff += 2.0 * PI;
    }

    if angle_diff.abs() > player.fov / 2.0 {
        return;
    }

    let sprite_d = ((player.pos.x - enemy.pos.x).powi(2) + (player.pos.y - enemy.pos.y).powi(2)).sqrt();

    // near plane           far plane
    if sprite_d < 50.0 || sprite_d > 1000.0 {
        return;
    }

    let screen_height = framebuffer.height as f32;
    let screen_width = framebuffer.width as f32;

    let sprite_size = (screen_height / sprite_d) * 70.0;
    let screen_x = ((angle_diff / player.fov) + 0.5) * screen_width;

    let start_x = (screen_x - sprite_size / 2.0).max(0.0) as usize;
    let start_y = (screen_height / 2.0 - sprite_size / 2.0).max(0.0) as usize;
    let sprite_size_usize = sprite_size as usize;
    let end_x = (start_x + sprite_size_usize).min(framebuffer.width as usize);
    let end_y = (start_y + sprite_size_usize).min(framebuffer.height as usize);

    for x in start_x..end_x {
        for y in start_y..end_y {
            let tx = ((x - start_x) * 128 / sprite_size_usize) as u32;
            let ty = ((y - start_y) * 128 / sprite_size_usize) as u32;

            let color = texture_manager.get_pixel_color(enemy.texture_key, tx, ty);
            
            if color != TRANSPARENT_COLOR {
                framebuffer.set_current_color(color);
                framebuffer.set_pixel(x as u32, y as u32);
            }
        }
    }
}