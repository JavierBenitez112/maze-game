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