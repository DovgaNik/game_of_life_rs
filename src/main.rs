use std::collections::{HashSet, VecDeque};
use rand::Rng;

const MAPSIZE: i128 = 1000;
const N_CELLS: usize = 1000;
const P_CELL: f64 = 0.8;

fn main() {
    let mut cells_alive_a: HashSet<(i128, i128)> = init_random_map(N_CELLS, MAPSIZE, MAPSIZE, P_CELL);
    let mut cells_alive_b: HashSet<(i128, i128)> = HashSet::new();

    let mut counter: u128 = 0;

    loop {
        println!("n_iter = {}, n_alive = {}", counter, cells_alive_a.len());

        let mut cells_to_check: HashSet<(i128, i128)> = HashSet::new();
        for &cell in &cells_alive_a {
            for neighbor in calculate_neighbors(cell) {
                cells_to_check.insert(neighbor);
            }
        }

        for &cell in &cells_to_check {
            let n_alive = check_cell(cell, &cells_alive_a);

            // Rule 1: live cell with fewer than two live neighbors dies
            // Rule 2: live cell with 2 or 3 lives
            // Rule 3: live cell with more than 3 dies
            // Rule 4: dead cell with three becomes alive
           
            if cells_alive_a.contains(&cell) {
                if n_alive == 2 || n_alive == 3 {
                    cells_alive_b.insert(cell);
                }
            } else {
                if n_alive == 3 {
                    cells_alive_b.insert(cell);
                }
            }
        }

        if cells_alive_a.iter().len() == 0 {
            break;
        }
        
        cells_alive_a = cells_alive_b.clone();
        cells_alive_b.clear();
        counter += 1;
    }
}

fn check_cell(cell: (i128, i128), cells_alive: &HashSet<(i128, i128)>) -> u8 {
    let neighbors = calculate_neighbors(cell);
    let mut n_alive = 0;

    for neighbor in neighbors {
        if cells_alive.contains(&neighbor) {
            n_alive += 1;
        }
    }
    n_alive
}

fn calculate_neighbors(cell: (i128, i128)) -> Vec<(i128, i128)> {
    vec![
        (cell.0 - 1, cell.1 - 1),
        (cell.0 - 1, cell.1),
        (cell.0 - 1, cell.1 + 1),
        (cell.0 + 1, cell.1 - 1),
        (cell.0 + 1, cell.1),
        (cell.0 + 1, cell.1 + 1),
        (cell.0, cell.1 - 1),
        (cell.0, cell.1 + 1),
    ]
}

fn init_map() -> HashSet<(i128, i128)> {
    let initial_cells = vec![
        (0, 1),
        (1, 1),
        (1, 0),
        (1, 2),
        (2, 2),
    ];
    initial_cells.into_iter().collect()
}

fn init_random_map(num_cells: usize, x_limit: i128, y_limit: i128, probability: f64) -> HashSet<(i128, i128)> {
    let mut rng = rand::thread_rng();
    let mut cells_alive = HashSet::new();

    for _ in 0..num_cells {
        let x = rng.gen_range(0..x_limit);
        let y = rng.gen_range(0..y_limit);

        if rng.gen_bool(probability) {
            cells_alive.insert((x, y));
        }
    }

    cells_alive
}