use std::collections::{HashSet, LinkedList};

const MAP_SIZE: usize = 25;
const T_FRAME: u64 = 100;

fn main() {
    let mut cells_alive_a: LinkedList<(i32, i32)> = init_map();
    let mut cells_alive_b: LinkedList<(i32, i32)> = init_map();

    let mut counter = 0;
    // Main loop
    loop {
        println!("n_alive = {}", cells_alive_a.len());
        let mut cells_to_check: LinkedList<(i32, i32)> = LinkedList::new();
        for &cell in &cells_alive_a {
             cells_to_check.append(&mut calculate_neighbors(cell));
        }
        remove_duplicates(&mut cells_to_check);
        
        for &cell in &cells_to_check {
            
            let n_alive: u8 = check_cell(cell, &cells_alive_a);
            
            // Rule 1: live cell with fewer than two live neighbors dies
            // Rule 2: live cell with 2 or 3 lives
            // Rule 3: live cell with more than 3 dies
            // Rule 4: dead cell with three becomes alive
            
            if check_element_in_list(cell, &cells_alive_a) {
                if n_alive == 2 || n_alive == 3 {
                    cells_alive_b.push_back(cell);
                }
            } else {
                if n_alive == 3 {
                    cells_alive_b.push_back(cell);
                }
            } 
            
        }
        
        cells_alive_a = cells_alive_b.clone();
        cells_alive_b = LinkedList::new();
        counter += 1;
        if counter > 50 {
            break
        } 
    } 

}

fn check_element_in_list (element: (i32, i32), list: &LinkedList<(i32, i32)>) -> bool {
    for &item in list {
        if (item == element) {
            return true;
        }
    }
    return false;
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

fn init_map() -> LinkedList<(i32, i32)> {
    let mut cells_alive: LinkedList<(i32, i32)> = LinkedList::new();

    cells_alive.push_back((0, 1));
    cells_alive.push_back((1, 1));
    cells_alive.push_back((1, 0));
    cells_alive.push_back((1, 2));
    cells_alive.push_back((2, 2));   
    
    return (cells_alive)
}

fn remove_duplicates(list: &mut LinkedList<(i32, i32)>) {
    let mut seen = HashSet::new();
    let mut unique_list = LinkedList::new();

    for &item in list.iter() {
        if seen.insert(item) {
            unique_list.push_back(item);
        }
    }

    *list = unique_list;
}