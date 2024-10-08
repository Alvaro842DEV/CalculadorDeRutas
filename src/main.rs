mod structures;
mod algorithms;
use crate::structures::Grafo;
use crate::algorithms::bidirectional::bidirectional_search;
use crate::algorithms::regions::partition_graph;
use std::fs::File;
use std::io::{self, Write};
use itertools::Itertools;

const SIZE: usize = 100;
const NUM_PARTITIONS: usize = 4;
const OUTPUT_FILE: &str = "resultado.txt";

#[derive(Clone, Copy, PartialEq, Debug)]
enum Cell {
    Empty,
    Obstacle,
    Path,
    Start,
    Goal,
}

fn main() -> io::Result<()> {
    let mut grid = initialize_empty_grid();

    File::create(OUTPUT_FILE)?;

    place_obstacles(&mut grid);
    let start = get_coordinates("inicio");
    let goal = get_coordinates("fin");

    println!("Buscando ruta de {:?} a {:?}", start, goal);
    match find_path(&mut grid.clone(), start, goal) {
        Some((path, cost)) => {
            println!("Ruta encontrada con costo {:.2}", cost);
            if let Err(e) = write_output(OUTPUT_FILE, &grid, &path, cost, start, goal) {
                eprintln!("Error al escribir el archivo de salida: {}", e);
            } else {
                println!("Resultado escrito en: {}", OUTPUT_FILE);
            }
        },
        None => {
            println!("No se encontró una ruta válida.");
            if let Err(e) = write_no_path(OUTPUT_FILE) {
                eprintln!("Error al escribir el archivo de salida: {}", e);
            } else {
                println!("Resultado escrito en: {}", OUTPUT_FILE);
            }
        }
    }

    Ok(())
}

fn initialize_empty_grid() -> [[Cell; SIZE]; SIZE] {
    [[Cell::Empty; SIZE]; SIZE]
}

fn place_obstacles(grid: &mut [[Cell; SIZE]; SIZE]) {
    println!("Colocación de obstáculos:");
    println!("Introduce las coordenadas de los obstáculos (x,y) separadas por comas.");
    println!("Escribe 'fin' cuando hayas terminado.");

    loop {
        print!("Coordenadas del obstáculo (o 'fin'): ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Error al leer la entrada");
        let input = input.trim();

        if input.to_lowercase() == "fin" {
            break;
        }

        let coords: Vec<usize> = input
            .split(',')
            .filter_map(|s| s.trim().parse().ok())
            .collect();

        if coords.len() == 2 && coords[0] < SIZE && coords[1] < SIZE {
            grid[coords[1]][coords[0]] = Cell::Obstacle;
            println!("Obstáculo colocado en ({}, {})", coords[0], coords[1]);
        } else {
            println!("Coordenadas inválidas. Por favor, introduce dos números entre 0 y {} separados por una coma.", SIZE - 1);
        }
    }
}

fn get_coordinates(point_type: &str) -> (usize, usize) {
    loop {
        println!("Introduce las coordenadas del punto de {} (x,y):", point_type);
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Error al leer la entrada");
        let coords: Vec<usize> = input
            .trim()
            .split(',')
            .filter_map(|s| s.parse().ok())
            .collect();

        if coords.len() == 2 && coords[0] < SIZE && coords[1] < SIZE {
            return (coords[0], coords[1]);
        } else {
            println!("Coordenadas inválidas. Por favor, introduce dos números entre 0 y {} separados por una coma.", SIZE - 1);
        }
    }
}

fn find_path(grid: &mut [[Cell; SIZE]; SIZE], start: (usize, usize), goal: (usize, usize)) -> Option<(Vec<(usize, usize)>, f64)> {
    let mut graph = create_graph_from_grid(grid);
    partition_graph(&mut graph, NUM_PARTITIONS);
    let start_index = start.1 * SIZE + start.0;
    let goal_index = goal.1 * SIZE + goal.0;

    bidirectional_search(&graph, start_index, goal_index)
}

fn write_output(filename: &str, grid: &[[Cell; SIZE]; SIZE], path: &[(usize, usize)], cost: f64, start: (usize, usize), goal: (usize, usize)) -> std::io::Result<()> {
    let mut file = File::create(filename)?;

    writeln!(file, "Informe de Ruta")?;
    writeln!(file, "===============\n")?;

    writeln!(file, "Información Básica:")?;
    writeln!(file, "-------------------")?;
    writeln!(file, "Punto de inicio: ({}, {})", start.0, start.1)?;
    writeln!(file, "Punto final: ({}, {})", goal.0, goal.1)?;
    writeln!(file, "Longitud de la ruta: {} pasos", path.len() - 1)?;
    writeln!(file, "Costo total: {:.2}", cost)?;

    writeln!(file, "\nEstadísticas del Camino:")?;
    writeln!(file, "-------------------------")?;
    if !path.is_empty() {
        let (min_x, max_x) = path.iter().map(|&(x, _)| x).minmax().into_option().unwrap();
        let (min_y, max_y) = path.iter().map(|&(_, y)| y).minmax().into_option().unwrap();
        writeln!(file, "Rango X: {} - {}", min_x, max_x)?;
        writeln!(file, "Rango Y: {} - {}", min_y, max_y)?;
    } else {
        writeln!(file, "El camino está vacío")?;
    }
    let manhattan_distance = (start.0 as i32 - goal.0 as i32).abs() + (start.1 as i32 - goal.1 as i32).abs();
    writeln!(file, "Distancia Manhattan entre inicio y fin: {}", manhattan_distance)?;
    writeln!(file, "Eficiencia de la ruta: {:.2}%", (manhattan_distance as f64 / cost) * 100.0)?;

    writeln!(file, "\nInformación del Grid:")?;
    writeln!(file, "---------------------")?;
    let obstacle_count = grid.iter().flatten().filter(|&&cell| cell == Cell::Obstacle).count();
    writeln!(file, "Tamaño del grid: {}x{}", SIZE, SIZE)?;
    writeln!(file, "Número de obstáculos: {}", obstacle_count)?;
    writeln!(file, "Densidad de obstáculos: {:.2}%", (obstacle_count as f64 / (SIZE * SIZE) as f64) * 100.0)?;

    writeln!(file, "\nRepresentación del Grid:")?;
    writeln!(file, "========================")?;

    let mut grid_with_path = grid.clone();
    for &(x, y) in path {
        if grid_with_path[y][x] == Cell::Empty {
            grid_with_path[y][x] = Cell::Path;
        }
    }

    let (start_x, start_y) = start;
    let (goal_x, goal_y) = goal;
    grid_with_path[start_y][start_x] = Cell::Start;
    grid_with_path[goal_y][goal_x] = Cell::Goal;

    for row in grid_with_path.iter() {
        for &cell in row.iter() {
            let symbol = match cell {
                Cell::Empty => '.',
                Cell::Obstacle => '#',
                Cell::Path => '*',
                Cell::Start => 'S',
                Cell::Goal => 'G',
            };
            write!(file, "{}", symbol)?;
        }
        writeln!(file)?;
    }

    writeln!(file, "\nCoordenadas de la Ruta:")?;
    writeln!(file, "========================")?;
    for (i, &(x, y)) in path.iter().enumerate() {
        writeln!(file, "Paso {}: ({}, {})", i, x, y)?;
    }

    Ok(())
}

fn write_no_path(filename: &str) -> std::io::Result<()> {
    let mut file = File::create(filename)?;
    writeln!(file, "No se encontró una ruta válida.")
}

fn create_graph_from_grid(grid: &[[Cell; SIZE]; SIZE]) -> Grafo {
    let mut graph = Grafo::new(SIZE * SIZE, (0..SIZE * SIZE).map(|i| (i % SIZE, i / SIZE)).collect());

    for y in 0..SIZE {
        for x in 0..SIZE {
            let index = y * SIZE + x;

            if grid[y][x] != Cell::Obstacle {
                let directions = [(0, -1), (0, 1), (-1, 0), (1, 0)];
                for (dx, dy) in directions.iter() {
                    let nx = x as i32 + *dx;
                    let ny = y as i32 + *dy;
                    if nx >= 0 && nx < SIZE as i32 && ny >= 0 && ny < SIZE as i32 {
                        let neighbor_index = ny as usize * SIZE + nx as usize;
                        if grid[ny as usize][nx as usize] != Cell::Obstacle {
                            graph.add_edge(index, neighbor_index, 1.0);
                        }
                    }
                }
            }
        }
    }

    graph
}
