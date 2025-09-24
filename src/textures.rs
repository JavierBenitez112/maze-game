// textures.rs

use raylib::prelude::*;
use std::collections::HashMap;

pub struct TextureManager {
    images: HashMap<char, Image>,       // Store images for pixel access
    textures: HashMap<char, Texture2D>, // Store GPU textures for rendering
}

impl TextureManager {
    pub fn new(rl: &mut RaylibHandle, thread: &RaylibThread) -> Self {
        let mut images = HashMap::new();
        let mut textures = HashMap::new();

        // Map characters to texture file paths
        let texture_files = vec![
            ('+', "assets/wallU.png"),
            ('-', "assets/wallU.png"),
            ('|', "assets/wallU.png"),
            ('g', "assets/wall5.png"),
            ('#', "assets/wall3.png"), // default/fallback
            ('e', "assets/SpookyBG.png"), // sprite de enemigo
        ];

        for (ch, path) in texture_files {
            let image = Image::load_image(path).expect(&format!("Failed to load image {}", path));
            let texture = rl.load_texture(thread, path).expect(&format!("Failed to load texture {}", path));
            images.insert(ch, image);
            textures.insert(ch, texture);
        }

        TextureManager { images, textures }
    }

    pub fn get_pixel_color(&self, ch: char, tx: u32, ty: u32) -> Color {
        if let Some(image) = self.images.get(&ch) {
            let x = tx.min(image.width as u32 - 1) as i32;
            let y = ty.min(image.height as u32 - 1) as i32;
            
            // Usar la función nativa de Raylib para obtener el color del píxel
            unsafe {
                let raylib_color = raylib::ffi::GetImageColor(
                    raylib::ffi::Image {
                        data: image.data,
                        width: image.width,
                        height: image.height,
                        mipmaps: image.mipmaps,
                        format: image.format,
                    },
                    x,
                    y
                );
                
                 // Manejar diferentes formatos de imagen
                 let alpha = match image.format {
                     4 => raylib_color.a, // PIXELFORMAT_UNCOMPRESSED_R8G8B8A8
                     3 => 255, // PIXELFORMAT_UNCOMPRESSED_R8G8B8 (sin alpha)
                     _ => 255, // Por defecto, completamente opaco
                 };
                
                Color::new(raylib_color.r, raylib_color.g, raylib_color.b, alpha)
            }
        } else {
            Color::WHITE
        }
    }

    pub fn get_texture(&self, ch: char) -> Option<&Texture2D> {
        self.textures.get(&ch)
    }

    pub fn get_texture_dimensions(&self, ch: char) -> Option<(u32, u32)> {
        self.images.get(&ch).map(|img| (img.width as u32, img.height as u32))
    }
}
