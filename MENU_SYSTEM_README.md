# Sistema de Menú para Maze Runner

## Descripción

He creado un sistema de menú reutilizable que permite renderizar imágenes PNG y manejar diferentes estados del juego. El sistema es completamente modular y fácil de extender.

## Características

### ✅ Funcionalidades Implementadas

1. **Sistema de Menú Reutilizable**
   - Renderizado de imágenes PNG como fondo
   - Opciones con imágenes personalizadas
   - Navegación con flechas del teclado
   - Selección con ENTER
   - Colores personalizables

2. **Estados del Juego**
   - Menú Principal
   - Jugando
   - Nivel Completado
   - Game Over
   - Selector de Niveles

3. **Menús Predefinidos**
   - Menú principal con opciones de juego
   - Selector de niveles con imágenes
   - Menú de éxito de nivel
   - Menú de game over

## Cómo Usar

### 1. Crear un Menú Personalizado

```rust
use menu::{Menu, MenuAction};

let mut menu = Menu::new("Mi Menú".to_string());

// Configurar imagen de fondo
menu.set_background_image("assets/mi_fondo.png".to_string());

// Configurar colores
menu.set_colors(
    Color::new(255, 215, 0, 255), // Título (dorado)
    Color::new(255, 100, 100, 255), // Selección (rojo)
    Color::WHITE, // Normal
    Color::new(0, 0, 0, 150), // Fondo
);

// Agregar opciones
menu.add_option("Opción 1".to_string(), MenuAction::StartLevel);
menu.add_option_with_image(
    "Opción con Imagen".to_string(),
    "assets/mi_imagen.png".to_string(),
    MenuAction::Custom("mi_accion".to_string()),
);
```

### 2. Renderizar el Menú

```rust
let mut menu_renderer = MenuRenderer::new(&mut window, &raylib_thread);

// En el bucle principal
menu_renderer.render_menu(&mut window, &raylib_thread, &menu);
```

### 3. Manejar Entrada del Usuario

```rust
fn handle_menu_input(window: &RaylibHandle, menu: &mut Menu, game_state: &mut GameState) {
    // Navegación
    if window.is_key_pressed(KeyboardKey::KEY_UP) {
        menu.move_up();
    }
    if window.is_key_pressed(KeyboardKey::KEY_DOWN) {
        menu.move_down();
    }
    
    // Selección
    if window.is_key_pressed(KeyboardKey::KEY_ENTER) {
        match menu.get_selected_action() {
            MenuAction::StartLevel => {
                *game_state = GameState::Playing;
            }
            MenuAction::Custom(action) => {
                match action.as_str() {
                    "mi_accion" => {
                        // Manejar acción personalizada
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }
}
```

## Estructura del Código

### Archivos Principales

- `src/menu.rs` - Sistema de menú completo
- `src/main.rs` - Integración con el juego principal

### Componentes del Sistema

1. **MenuOption** - Representa una opción del menú
2. **MenuAction** - Enum de acciones disponibles
3. **Menu** - Estructura principal del menú
4. **MenuRenderer** - Renderizador de menús
5. **Funciones de utilidad** - Crear menús predefinidos

## Acciones Disponibles

- `StartLevel` - Iniciar nivel
- `Exit` - Salir del juego
- `NextLevel` - Siguiente nivel
- `Restart` - Reiniciar nivel
- `MainMenu` - Volver al menú principal
- `Custom(String)` - Acción personalizada

## Menús Predefinidos

### Menú Principal
```rust
let menu = create_main_menu();
```
- Empezar Juego
- Seleccionar Nivel
- Salir

### Selector de Niveles
```rust
let menu = create_level_selector_menu();
```
- Nivel 1 - Tutorial
- Nivel 2 - Desafío
- Nivel 3 - Experto
- Menú Principal

### Menú de Éxito
```rust
let menu = create_level_complete_menu();
```
- Siguiente Nivel
- Reiniciar
- Menú Principal

### Menú de Game Over
```rust
let menu = create_game_over_menu();
```
- Reintentar
- Menú Principal
- Salir

## Personalización

### Colores
Puedes personalizar los colores del menú:
```rust
menu.set_colors(
    title_color,    // Color del título
    selected_color, // Color de la opción seleccionada
    normal_color,   // Color de las opciones normales
    background_color, // Color de fondo
);
```

### Imágenes
- **Imagen de fondo**: Se escala automáticamente para cubrir toda la pantalla
- **Imágenes de opciones**: Se muestran a la izquierda del texto (50x50 píxeles)

## Controles

- **Flechas ↑↓**: Navegar por las opciones
- **ENTER**: Seleccionar opción
- **ESC** (en juego): Volver al menú principal

## Extensión del Sistema

Para agregar nuevos menús o funcionalidades:

1. **Nuevo menú**: Crear función similar a `create_main_menu()`
2. **Nueva acción**: Agregar variante al enum `MenuAction`
3. **Nueva funcionalidad**: Extender `MenuRenderer` o `Menu`

## Ejemplo de Uso Completo

```rust
// Crear menú
let mut current_menu = create_main_menu();
let mut menu_renderer = MenuRenderer::new(&mut window, &raylib_thread);
let mut game_state = GameState::MainMenu;

// En el bucle principal
match game_state {
    GameState::MainMenu | GameState::LevelComplete | GameState::GameOver => {
        handle_menu_input(&window, &mut current_menu, &mut game_state);
        menu_renderer.render_menu(&mut window, &raylib_thread, &current_menu);
    }
    GameState::Playing => {
        // Lógica del juego
    }
}
```

## Notas Técnicas

- Las imágenes se cargan dinámicamente (sin cache por simplicidad)
- El sistema es compatible con la API de Raylib 5.5.1
- Todas las funciones están documentadas en español
- El código sigue las convenciones de Rust

¡El sistema está listo para usar y es completamente reutilizable para cualquier pantalla de menú que necesites!
