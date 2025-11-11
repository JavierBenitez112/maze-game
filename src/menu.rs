use raylib::prelude::*;

#[derive(Debug, Clone)]
pub struct MenuOption {
    pub text: String,
    pub image_path: Option<String>,
    pub action: MenuAction,
}

#[derive(Debug, Clone)]
pub enum MenuAction {
    StartLevel,
    Exit,
    NextLevel,
    Restart,
    MainMenu,
    Custom(String), // Para acciones personalizadas
}

#[derive(Debug)]
pub struct Menu {
    pub title: String,
    pub options: Vec<MenuOption>,
    pub selected_index: usize,
    pub background_image: Option<String>,
    pub title_color: Color,
    pub selected_color: Color,
    pub normal_color: Color,
    pub background_color: Color,
}

impl Menu {
    pub fn new(title: String) -> Self {
        Self {
            title,
            options: Vec::new(),
            selected_index: 0,
            background_image: None,
            title_color: Color::WHITE,
            selected_color: Color::YELLOW,
            normal_color: Color::WHITE,
            background_color: Color::new(0, 0, 0, 200),
        }
    }

    pub fn add_option(&mut self, text: String, action: MenuAction) {
        self.options.push(MenuOption {
            text,
            image_path: None,
            action,
        });
    }

    pub fn add_option_with_image(&mut self, text: String, image_path: String, action: MenuAction) {
        self.options.push(MenuOption {
            text,
            image_path: Some(image_path),
            action,
        });
    }

    pub fn set_background_image(&mut self, image_path: String) {
        self.background_image = Some(image_path);
    }

    pub fn set_colors(&mut self, title: Color, selected: Color, normal: Color, background: Color) {
        self.title_color = title;
        self.selected_color = selected;
        self.normal_color = normal;
        self.background_color = background;
    }

    pub fn move_up(&mut self) {
        if self.selected_index > 0 {
            self.selected_index -= 1;
        } else {
            self.selected_index = self.options.len() - 1;
        }
    }

    pub fn move_down(&mut self) {
        if self.selected_index < self.options.len() - 1 {
            self.selected_index += 1;
        } else {
            self.selected_index = 0;
        }
    }

    pub fn get_selected_action(&self) -> &MenuAction {
        &self.options[self.selected_index].action
    }

    pub fn get_selected_text(&self) -> &String {
        &self.options[self.selected_index].text
    }
}

pub struct MenuRenderer {
    // Simplificado - sin cache por ahora
}

impl MenuRenderer {
    pub fn new(_window: &mut RaylibHandle, _raylib_thread: &RaylibThread) -> Self {
        Self {}
    }

    pub fn render_menu(&mut self, window: &mut RaylibHandle, raylib_thread: &RaylibThread, menu: &Menu) {
        let screen_width = window.get_screen_width();
        let screen_height = window.get_screen_height();
        let mut d = window.begin_drawing(&raylib_thread);

        // Dibujar fondo con color
        d.clear_background(menu.background_color);

        // Dibujar título
        let title_font_size = 60;
        let title_width = d.measure_text(&menu.title, title_font_size);
        let title_x = (screen_width - title_width) / 2;
        let title_y = 100;
        
        d.draw_text(
            &menu.title,
            title_x,
            title_y,
            title_font_size,
            menu.title_color,
        );

        // Dibujar opciones
        let option_font_size = 40;
        let option_spacing = 80;
        let start_y = title_y + 150;
        let center_x = screen_width / 2;

        for (index, option) in menu.options.iter().enumerate() {
            let y = start_y + (index as i32 * option_spacing);
            let is_selected = index == menu.selected_index;
            let color = if is_selected { menu.selected_color } else { menu.normal_color };

            // Dibujar texto de la opción
            let text_width = d.measure_text(&option.text, option_font_size);
            let text_x = center_x - (text_width / 2);
            
            // Dibujar indicador de selección
            if is_selected {
                d.draw_text(">", text_x - 30, y, option_font_size, menu.selected_color);
                d.draw_text("<", text_x + text_width + 10, y, option_font_size, menu.selected_color);
            }
            
            d.draw_text(
                &option.text,
                text_x,
                y,
                option_font_size,
                color,
            );
        }

        // Dibujar instrucciones
        let instruction_text = "Usa las flechas para navegar y ENTER para seleccionar";
        let instruction_font_size = 20;
        let instruction_width = d.measure_text(instruction_text, instruction_font_size);
        let instruction_x = (screen_width - instruction_width) / 2;
        let instruction_y = screen_height - 50;
        
        d.draw_text(
            instruction_text,
            instruction_x,
            instruction_y,
            instruction_font_size,
            Color::GRAY,
        );
    }
}

// Funciones de utilidad para crear menús específicos
pub fn create_main_menu() -> Menu {
    let mut menu = Menu::new("Maze Runner".to_string());
    menu.set_background_image("assets/SpookyBG.png".to_string());
    menu.set_colors(
        Color::new(255, 215, 0, 255), // Dorado para el título
        Color::new(255, 100, 100, 255), // Rojo para selección
        Color::WHITE,
        Color::new(0, 0, 0, 150),
    );
    
    menu.add_option_with_image(
        "Empezar Juego".to_string(),
        "assets/SpookyNormal.png".to_string(),
        MenuAction::StartLevel,
    );
    menu.add_option_with_image(
        "Seleccionar Nivel".to_string(),
        "assets/enemybg.png".to_string(),
        MenuAction::Custom("level_selector".to_string()),
    );
    menu.add_option("Salir".to_string(), MenuAction::Exit);
    
    menu
}

pub fn create_level_complete_menu() -> Menu {
    let mut menu = Menu::new("¡Nivel Completado!".to_string());
    menu.set_background_image("assets/SpookyBG.png".to_string());
    menu.set_colors(
        Color::new(0, 255, 0, 255), // Verde para éxito
        Color::new(255, 215, 0, 255), // Dorado para selección
        Color::WHITE,
        Color::new(0, 0, 0, 150),
    );
    
    menu.add_option("Siguiente Nivel".to_string(), MenuAction::NextLevel);
    menu.add_option("Reiniciar".to_string(), MenuAction::Restart);
    menu.add_option("Menú Principal".to_string(), MenuAction::MainMenu);
    
    menu
}

pub fn create_game_over_menu() -> Menu {
    let mut menu = Menu::new("Game Over".to_string());
    menu.set_background_image("assets/SpookyBG.png".to_string());
    menu.set_colors(
        Color::new(255, 0, 0, 255), // Rojo para game over
        Color::new(255, 215, 0, 255), // Dorado para selección
        Color::WHITE,
        Color::new(0, 0, 0, 150),
    );
    
    menu.add_option("Reintentar".to_string(), MenuAction::Restart);
    menu.add_option("Menú Principal".to_string(), MenuAction::MainMenu);
    menu.add_option("Salir".to_string(), MenuAction::Exit);
    
    menu
}

pub fn create_level_selector_menu() -> Menu {
    let mut menu = Menu::new("Seleccionar Nivel".to_string());
    menu.set_background_image("assets/SpookyBG.png".to_string());
    menu.set_colors(
        Color::new(100, 200, 255, 255), // Azul claro para selector
        Color::new(255, 215, 0, 255), // Dorado para selección
        Color::WHITE,
        Color::new(0, 0, 0, 150),
    );
    
    // Agregar niveles con imágenes
    menu.add_option_with_image(
        "Nivel 1 - Tutorial".to_string(),
        "assets/SpookyNormal.png".to_string(),
        MenuAction::Custom("level_1".to_string()),
    );
    menu.add_option_with_image(
        "Nivel 2 - Desafío".to_string(),
        "assets/enemybg.png".to_string(),
        MenuAction::Custom("level_2".to_string()),
    );
    menu.add_option_with_image(
        "Nivel 3 - Experto".to_string(),
        "assets/brick.png".to_string(),
        MenuAction::Custom("level_3".to_string()),
    );
    menu.add_option("Menú Principal".to_string(), MenuAction::MainMenu);
    
    menu
}
