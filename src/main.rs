use std::collections::{HashSet, VecDeque};

fn main() {
    let mut cells_alive_a: HashSet<(i128, i128)> = init_map();
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