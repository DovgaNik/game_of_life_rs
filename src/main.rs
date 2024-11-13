use rand::Rng;
use std::thread::{self, sleep};
use std::time::Duration;
use std::collections::{LinkedList};

const MAP_SIZE: usize = 25;
const T_FRAME: u64 = 100;

fn main() {
    let mut cells_alive_a: LinkedList<(i32, i32)> = LinkedList::new();
    let mut cells_alive_b: LinkedList<(i32, i32)> = LinkedList::new();

    cells_alive_a.push_back((0, 1));
    cells_alive_a.push_back((1, 1));
    cells_alive_a.push_back((1, 0));
    cells_alive_a.push_back((1, 2));
    cells_alive_a.push_back((2, 2));

    for &cell in &cells_alive_a {
        let mut n_alive: u8 = 0;
        let mut neighbors: LinkedList<(i32, i32)> = calculate_neighbors(cell);
        
    }

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