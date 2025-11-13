// screens.rs - Sistema de pantallas reutilizable

use raylib::prelude::*;
use crate::framebuffer::Framebuffer;

pub enum ScreenType {
    MainMenu,
    Victory,
}

pub enum MenuState {
    MainMenu,
    LevelSelect,
}

pub struct ScreenManager {
    background_texture: Option<Texture2D>,
    background_image: Option<Image>,
    victory_texture: Option<Texture2D>,
    victory_image: Option<Image>,
    current_level: usize,
    menu_state: MenuState,
    selected_option: usize, // 0: Comenzar, 1: Selector de niveles, 2: Salir
}

impl ScreenManager {
    pub fn new(window: &mut RaylibHandle, raylib_thread: &RaylibThread) -> Self {
        // Cargar imagen de fondo del menú
        let background_image = Image::load_image("assets/spookyspn.png").ok();
        let background_texture = background_image.as_ref().and_then(|img| {
            window.load_texture_from_image(raylib_thread, img).ok()
        });

        // Cargar imagen de victoria (door.png)
        let victory_image = Image::load_image("assets/door.png").ok();
        let victory_texture = victory_image.as_ref().and_then(|img| {
            window.load_texture_from_image(raylib_thread, img).ok()
        });

        ScreenManager {
            background_texture,
            background_image,
            victory_texture,
            victory_image,
            current_level: 1,
            menu_state: MenuState::MainMenu,
            selected_option: 0,
        }
    }

    pub fn get_level_file(&self, level: usize) -> &str {
        match level {
            1 => "maze.txt",
            2 => "maze2.txt",
            3 => "maze3.txt",
            _ => "maze.txt",
        }
    }

    pub fn set_current_level(&mut self, level: usize) {
        self.current_level = level;
    }

    pub fn get_current_level(&self) -> usize {
        self.current_level
    }

    pub fn get_menu_state(&self) -> &MenuState {
        &self.menu_state
    }

    pub fn set_menu_state(&mut self, state: MenuState) {
        self.menu_state = state;
    }

    pub fn get_selected_option(&self) -> usize {
        self.selected_option
    }

    pub fn set_selected_option(&mut self, option: usize) {
        self.selected_option = option;
    }
}

pub fn render_screen(
    framebuffer: &mut Framebuffer,
    _window: &mut RaylibHandle,
    _raylib_thread: &RaylibThread,
    screen_type: ScreenType,
    screen_manager: &ScreenManager,
) {
    let screen_width = framebuffer.width;
    let screen_height = framebuffer.height;

    // Dibujar fondo
    if let Some(ref bg_texture) = screen_manager.background_texture {
        // Dibujar imagen de fondo escalada
        let bg_width = bg_texture.width as f32;
        let bg_height = bg_texture.height as f32;
        let scale_x = screen_width as f32 / bg_width;
        let scale_y = screen_height as f32 / bg_height;
        let scale = scale_x.min(scale_y);

        let scaled_width = bg_width * scale;
        let scaled_height = bg_height * scale;
        let x = (screen_width as f32 - scaled_width) / 2.0;
        let y = (screen_height as f32 - scaled_height) / 2.0;

        // Dibujar en el framebuffer usando la imagen
        if let Some(ref bg_image) = screen_manager.background_image {
            for py in 0..(scaled_height as u32).min(screen_height) {
                for px in 0..(scaled_width as u32).min(screen_width) {
                    let src_x = ((px as f32 / scale) as u32).min(bg_width as u32 - 1);
                    let src_y = ((py as f32 / scale) as u32).min(bg_height as u32 - 1);
                    
                    unsafe {
                        let raylib_color = raylib::ffi::GetImageColor(
                            raylib::ffi::Image {
                                data: bg_image.data,
                                width: bg_image.width,
                                height: bg_image.height,
                                mipmaps: bg_image.mipmaps,
                                format: bg_image.format,
                            },
                            src_x as i32,
                            src_y as i32
                        );
                        
                        let alpha = match bg_image.format {
                            4 => raylib_color.a,
                            3 => 255,
                            _ => 255,
                        };
                        
                        let color = Color::new(raylib_color.r, raylib_color.g, raylib_color.b, alpha);
                        framebuffer.set_pixel_color((x as u32 + px).min(screen_width - 1), (y as u32 + py).min(screen_height - 1), color);
                    }
                }
            }
        }
    } else {
        // Si no hay imagen, usar fondo negro
        framebuffer.set_background_color(Color::BLACK);
        framebuffer.clear();
    }

        match screen_type {
        ScreenType::MainMenu => {
            render_main_menu(framebuffer, screen_width, screen_height, screen_manager);
        }
        ScreenType::Victory => {
            render_victory_screen(framebuffer, screen_width, screen_height, screen_manager);
        }
    }
}

fn render_victory_background(
    framebuffer: &mut Framebuffer,
    screen_width: u32,
    screen_height: u32,
    screen_manager: &ScreenManager,
) {
    // Dibujar imagen de fondo de victoria
    if let Some(ref victory_image) = screen_manager.victory_image {
        let bg_width = victory_image.width as f32;
        let bg_height = victory_image.height as f32;
        let scale_x = screen_width as f32 / bg_width;
        let scale_y = screen_height as f32 / bg_height;
        let scale = scale_x.min(scale_y);

        let scaled_width = bg_width * scale;
        let scaled_height = bg_height * scale;
        let x = (screen_width as f32 - scaled_width) / 2.0;
        let y = (screen_height as f32 - scaled_height) / 2.0;

        // Dibujar en el framebuffer usando la imagen
        for py in 0..(scaled_height as u32).min(screen_height) {
            for px in 0..(scaled_width as u32).min(screen_width) {
                let src_x = ((px as f32 / scale) as u32).min(bg_width as u32 - 1);
                let src_y = ((py as f32 / scale) as u32).min(bg_height as u32 - 1);
                
                unsafe {
                    let raylib_color = raylib::ffi::GetImageColor(
                        raylib::ffi::Image {
                            data: victory_image.data,
                            width: victory_image.width,
                            height: victory_image.height,
                            mipmaps: victory_image.mipmaps,
                            format: victory_image.format,
                        },
                        src_x as i32,
                        src_y as i32
                    );
                    
                    let alpha = match victory_image.format {
                        4 => raylib_color.a,
                        3 => 255,
                        _ => 255,
                    };
                    
                    let color = Color::new(raylib_color.r, raylib_color.g, raylib_color.b, alpha);
                    framebuffer.set_pixel_color((x as u32 + px).min(screen_width - 1), (y as u32 + py).min(screen_height - 1), color);
                }
            }
        }
    } else {
        // Si no hay imagen, usar fondo negro
        framebuffer.set_background_color(Color::BLACK);
        framebuffer.clear();
    }
}

fn render_main_menu(
    framebuffer: &mut Framebuffer,
    width: u32,
    height: u32,
    screen_manager: &ScreenManager,
) {
    let center_x = width / 2;
    let center_y = height / 2;

    // Dibujar título (simulado con rectángulos, ya que no tenemos renderizado de texto en framebuffer)
    // Por ahora, usaremos un método simple de dibujar texto básico
    framebuffer.set_current_color(Color::WHITE);
    // Nota: Para texto real, necesitaríamos usar raylib directamente en swap_buffer
    // Por ahora, dibujamos un indicador visual

    // Opciones de nivel
    let level_options = vec!["Nivel 1", "Nivel 2", "Nivel 3"];
    let option_spacing = 80;
    let start_y = center_y - 50;

    for (i, _level_text) in level_options.iter().enumerate() {
        let y = start_y + (i as u32 * option_spacing);
        let is_selected = (i + 1) == screen_manager.get_current_level();

        // Dibujar indicador de selección
        if is_selected {
            framebuffer.set_current_color(Color::YELLOW);
            // Dibujar un rectángulo alrededor de la opción seleccionada
            for px in (center_x - 100)..(center_x + 100) {
                for py in (y - 5)..(y + 25) {
                    if px < width && py < height {
                        framebuffer.set_pixel(px, py);
                    }
                }
            }
        } else {
            framebuffer.set_current_color(Color::GRAY);
        }

        // Dibujar indicador de opción (círculo pequeño)
        for dx in -3..=3 {
            for dy in -3..=3 {
                let px = (center_x as i32 - 120 + dx).max(0) as u32;
                let py = (y as i32 + 10 + dy).max(0) as u32;
                if px < width && py < height && (dx * dx + dy * dy) <= 9 {
                    framebuffer.set_pixel(px, py);
                }
            }
        }
    }

    // Instrucciones (dibujadas visualmente)
    framebuffer.set_current_color(Color::WHITE);
    // Indicador de instrucciones en la parte inferior
    for px in (center_x - 200)..(center_x + 200) {
        for py in (height - 30)..(height - 20) {
            if px < width && py < height {
                framebuffer.set_pixel(px, py);
            }
        }
    }
}

fn render_victory_screen(
    framebuffer: &mut Framebuffer,
    width: u32,
    height: u32,
    screen_manager: &ScreenManager,
) {
    // Dibujar fondo de victoria (pic0.png)
    render_victory_background(framebuffer, width, height, screen_manager);
}

pub enum MenuAction {
    StartLevel(usize),
    Exit,
    None,
}

pub enum VictoryAction {
    NextLevel,
    MainMenu,
    None,
}

pub fn handle_menu_input(window: &RaylibHandle, screen_manager: &mut ScreenManager) -> Option<MenuAction> {
    match screen_manager.get_menu_state() {
        MenuState::MainMenu => {
            // Navegación en menú principal
            if window.is_key_pressed(KeyboardKey::KEY_UP) {
                let current = screen_manager.get_selected_option();
                if current > 0 {
                    screen_manager.set_selected_option(current - 1);
                }
                return None;
            }

            if window.is_key_pressed(KeyboardKey::KEY_DOWN) {
                let current = screen_manager.get_selected_option();
                if current < 2 {
                    screen_manager.set_selected_option(current + 1);
                }
                return None;
            }

            // Seleccionar opción con Enter
            if window.is_key_pressed(KeyboardKey::KEY_ENTER) {
                match screen_manager.get_selected_option() {
                    0 => {
                        // Comenzar - iniciar nivel 1
                        screen_manager.set_current_level(1);
                        return Some(MenuAction::StartLevel(1));
                    }
                    1 => {
                        // Selector de niveles
                        screen_manager.set_menu_state(MenuState::LevelSelect);
                        screen_manager.set_current_level(1);
                        return None;
                    }
                    2 => {
                        // Salir
                        return Some(MenuAction::Exit);
                    }
                    _ => {}
                }
            }
        }
        MenuState::LevelSelect => {
            // Navegación en selector de niveles
            if window.is_key_pressed(KeyboardKey::KEY_UP) {
                let current = screen_manager.get_current_level();
                if current > 1 {
                    screen_manager.set_current_level(current - 1);
                }
                return None;
            }

            if window.is_key_pressed(KeyboardKey::KEY_DOWN) {
                let current = screen_manager.get_current_level();
                if current < 3 {
                    screen_manager.set_current_level(current + 1);
                }
                return None;
            }

            // Seleccionar nivel con Enter
            if window.is_key_pressed(KeyboardKey::KEY_ENTER) {
                return Some(MenuAction::StartLevel(screen_manager.get_current_level()));
            }

            // Volver al menú principal con ESC
            if window.is_key_pressed(KeyboardKey::KEY_ESCAPE) {
                screen_manager.set_menu_state(MenuState::MainMenu);
                return None;
            }
        }
    }

    None
}

pub fn handle_victory_input(window: &RaylibHandle) -> Option<VictoryAction> {
    // Enter para siguiente nivel
    if window.is_key_pressed(KeyboardKey::KEY_ENTER) {
        return Some(VictoryAction::NextLevel);
    }

    // ESC para menú principal
    if window.is_key_pressed(KeyboardKey::KEY_ESCAPE) {
        return Some(VictoryAction::MainMenu);
    }

    None
}

