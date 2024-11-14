use std::collections::{LinkedList};

const MAP_SIZE: usize = 25;
const T_FRAME: u64 = 100;

fn main() {
    let mut cells_alive_a: LinkedList<(i32, i32)> = LinkedList::new();
    let mut cells_alive_b: LinkedList<(i32, i32)>;

    cells_alive_a.push_back((0, 1));
    cells_alive_a.push_back((1, 1));
    cells_alive_a.push_back((1, 0));
    cells_alive_a.push_back((1, 2));
    cells_alive_a.push_back((2, 2));

    for &cell in &cells_alive_a {
        check_cell(cell, &cells_alive_a);
    }

}

fn check_cell(cell: (i32, i32), cells_alive: &LinkedList<(i32, i32)>) -> u8 {
    let mut n_alive: u8 = 0;
    let neighbors: LinkedList<(i32, i32)> = calculate_neighbors(cell);

    for neighboring_cell in cells_alive {
        for neighbor in &neighbors {
            if (*neighboring_cell == *neighbor) {
                n_alive += 1;
            }
        }
    }
    println!("({}, {}) n_alive={}", cell.0, cell.1, n_alive);
    return(n_alive);
}

fn calculate_neighbors (cell: (i32, i32)) -> LinkedList<(i32, i32)> {
    let mut neighbors: LinkedList<(i32, i32)> = LinkedList::new();
    neighbors.push_back((cell.0 - 1, cell.1 - 1));
    neighbors.push_back((cell.0 - 1, cell.1));
    neighbors.push_back((cell.0 - 1, cell.1 + 1));
    neighbors.push_back((cell.0 + 1, cell.1 - 1));
    neighbors.push_back((cell.0 + 1, cell.1));
    neighbors.push_back((cell.0 + 1, cell.1 + 1));
    neighbors.push_back((cell.0, cell.1 - 1));
    neighbors.push_back((cell.0, cell.1 + 1));
    return (neighbors);
}