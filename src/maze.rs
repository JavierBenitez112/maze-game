use std::fs::File;
use std::io::{BufRead, BufReader};

pub type Maze = Vec<Vec<char>>;

pub fn load_maze(filename: &str) -> Vec<Vec<char>> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    reader
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect()
}

// Encontrar la posición inicial del jugador (carácter 'p')
// Busca una celda vacía adyacente al 'p' para colocar al jugador
pub fn find_player_start(maze: &Maze) -> Option<(f32, f32)> {
    let block_size = 100.0;
    
    // Primero encontrar la posición del 'p'
    for (row_index, row) in maze.iter().enumerate() {
        for (col_index, &cell) in row.iter().enumerate() {
            if cell == 'p' {
                // Buscar una celda vacía adyacente (arriba, abajo, izquierda, derecha)
                let directions = vec![
                    (0, -1),  // Arriba
                    (0, 1),   // Abajo
                    (-1, 0),  // Izquierda
                    (1, 0),   // Derecha
                ];
                
                for (dx, dy) in directions {
                    let new_col = col_index as i32 + dx;
                    let new_row = row_index as i32 + dy;
                    
                    // Verificar límites
                    if new_row >= 0 && new_row < maze.len() as i32 &&
                       new_col >= 0 && new_col < maze[0].len() as i32 {
                        let cell_char = maze[new_row as usize][new_col as usize];
                        // Si es una celda vacía, colocar al jugador en el centro de esa celda
                        if cell_char == ' ' {
                            let x = (new_col as f32 * block_size) + (block_size / 2.0);
                            let y = (new_row as f32 * block_size) + (block_size / 2.0);
                            return Some((x, y));
                        }
                    }
                }
                
                // Si no se encuentra una celda vacía adyacente, usar la posición del 'p' con offset
                // Esto es un fallback
                let x = (col_index as f32 * block_size) + (block_size / 2.0);
                let y = (row_index as f32 * block_size) + (block_size / 2.0);
                return Some((x, y));
            }
        }
    }
    None
}

// Verificar colisión con la meta (carácter 'g')
// Amplía el área de detección para hacer más fácil activar la victoria
pub fn check_goal_collision(maze: &Maze, player_x: f32, player_y: f32, block_size: usize) -> bool {
    let block_size_f = block_size as f32;
    
    // Buscar todas las posiciones donde hay una 'g' en el laberinto
    for (row_index, row) in maze.iter().enumerate() {
        for (col_index, &cell) in row.iter().enumerate() {
            if cell == 'g' {
                // Calcular el centro del bloque 'g'
                let goal_center_x = (col_index as f32 * block_size_f) + (block_size_f / 2.0);
                let goal_center_y = (row_index as f32 * block_size_f) + (block_size_f / 2.0);
                
                // Calcular la distancia del jugador al centro del bloque 'g'
                let dx = player_x - goal_center_x;
                let dy = player_y - goal_center_y;
                let distance = (dx * dx + dy * dy).sqrt();
                
                // Ampliar el área de detección: usar 70% del tamaño del bloque como radio
                // Esto hace que sea más fácil activar la victoria
                let detection_radius = block_size_f * 0.7;
                
                // Si el jugador está dentro del radio ampliado, activar victoria
                if distance <= detection_radius {
                    return true;
                }
            }
        }
    }
    
    false
}

// Función para verificar colisiones con las paredes
pub fn check_collision(maze: &Maze, new_x: f32, new_y: f32, block_size: usize) -> bool {
    let grid_x = (new_x / block_size as f32) as usize;
    let grid_y = (new_y / block_size as f32) as usize;
    
    // Verificar límites del laberinto
    if grid_x >= maze[0].len() || grid_y >= maze.len() {
        return true; // Colisión con límites
    }
    
    let cell = maze[grid_y][grid_x];
    
    // Los triggers ('t', 's', 'c') no tienen colisión, se pueden atravesar
    // Optimización: usar match para mejor rendimiento
    match cell {
        't' | 's' | 'c' => return false,
        _ => {}
    }
    
    // Verificar si hay una pared en la nueva posición
    cell != ' '
}

// Función para verificar colisiones con margen de seguridad
pub fn check_collision_with_margin(maze: &Maze, x: f32, y: f32, block_size: usize, margin: f32) -> bool {
    // Verificar múltiples puntos alrededor de la entidad para evitar que se pegue a las paredes
    let points = vec![
        (x - margin, y - margin), // Esquina superior izquierda
        (x + margin, y - margin), // Esquina superior derecha
        (x - margin, y + margin), // Esquina inferior izquierda
        (x + margin, y + margin), // Esquina inferior derecha
    ];
    
    for (px, py) in points {
        if check_collision(maze, px, py, block_size) {
            return true;
        }
    }
    
    false
}

// Función para verificar si hay línea de visión entre dos puntos (sin paredes)
pub fn has_line_of_sight(maze: &Maze, from_x: f32, from_y: f32, to_x: f32, to_y: f32, block_size: usize) -> bool {
    let dx = to_x - from_x;
    let dy = to_y - from_y;
    let distance = (dx * dx + dy * dy).sqrt();
    
    // Si está muy cerca, asumir que hay línea de visión
    if distance < 10.0 {
        return true;
    }
    
    // Calcular dirección normalizada (no se usa directamente, pero se calcula para consistencia)
    let _dir_x = dx / distance;
    let _dir_y = dy / distance;
    
    // Verificar puntos a lo largo de la línea usando step_size pequeño para precisión
    let step_size = 5.0;
    let num_steps = (distance / step_size) as usize + 1;
    
    for step in 0..num_steps {
        let t = (step as f32 * step_size) / distance;
        let t = t.min(1.0); // Asegurar que no exceda 1.0
        
        // Calcular posición actual en la línea
        let current_x = from_x + dx * t;
        let current_y = from_y + dy * t;
        
        // Verificar si hay una pared en este punto
        if check_collision(maze, current_x, current_y, block_size) {
            return false; // Hay una pared bloqueando la visión
        }
    }
    
    true // No hay paredes bloqueando la visión
}

// Función para encontrar triggers activados (cuando el jugador pasa por ellos)
// Retorna un vector de (posición x, posición y, carácter del trigger) de los triggers que el jugador está activando
pub fn find_activated_triggers(maze: &Maze, player_x: f32, player_y: f32, block_size: usize) -> Vec<(f32, f32, char)> {
    let block_size_f = block_size as f32;
    let mut activated = Vec::new();
    
    // Lista de caracteres que son triggers
    let trigger_chars = vec!['t', 's', 'c'];
    
    // Buscar todas las posiciones donde hay un trigger en el laberinto
    for (row_index, row) in maze.iter().enumerate() {
        for (col_index, &cell) in row.iter().enumerate() {
            if trigger_chars.contains(&cell) {
                // Calcular el centro del bloque trigger
                let trigger_center_x = (col_index as f32 * block_size_f) + (block_size_f / 2.0);
                let trigger_center_y = (row_index as f32 * block_size_f) + (block_size_f / 2.0);
                
                // Calcular la distancia del jugador al centro del trigger
                let dx = player_x - trigger_center_x;
                let dy = player_y - trigger_center_y;
                let distance = (dx * dx + dy * dy).sqrt();
                
                // Usar 70% del tamaño del bloque como radio de detección
                let detection_radius = block_size_f * 0.7;
                
                // Si el jugador está dentro del radio, el trigger está activado
                if distance <= detection_radius {
                    activated.push((trigger_center_x, trigger_center_y, cell));
                }
            }
        }
    }
    
    activated
}