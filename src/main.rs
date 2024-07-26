use rand::Rng;
use std::thread::{self, sleep};
use std::time::Duration;

const MAP_SIZE: usize = 20;
const P_TRUE: f64 = 0.3;

fn main() {
    let mut a: [[bool; MAP_SIZE]; MAP_SIZE] = [[false; MAP_SIZE]; MAP_SIZE];
    let mut b: [[bool; MAP_SIZE]; MAP_SIZE] = [[false; MAP_SIZE]; MAP_SIZE];

    init_map(&mut a);

    loop {
        print_screen(a);

        for x in 0..MAP_SIZE {
            for y in 0..MAP_SIZE {
                let mut n_alive: u8 = 0;

                let x_i;
                let y_i;

                if x > 0 {
                    x_i = x - 1;
                } else {
                    x_i = 0;
                }

                if y > 0 {
                    y_i = y - 1;
                } else {
                    y_i = 0;
                }

                let x_n;
                let y_n;

                if x < MAP_SIZE - 1 {
                    x_n = x + 2;
                } else {
                    x_n = MAP_SIZE;
                }

                if y < MAP_SIZE - 1 {
                    y_n = y + 2;
                } else {
                    y_n = MAP_SIZE;
                }

                for x_ in x_i..x_n {
                    for y_ in y_i..y_n {
                        if a[x_][y_] == true && (x_ != x || y_ != y) {
                            n_alive += 1;
                        }
                    }
                }

                // Rule 1: If a live cell has fewer then 2 neighboring live cells it dies
                if a[x][y] == true && n_alive < 2 {
                    b[x][y] = false;
                }

                // Rule 2: If a live cell has 2 or 3 neighboring live cells it continues to live
                if a[x][y] == true && (n_alive == 2 || n_alive == 3) {
                    b[x][y] = true;
                }

                // Rule 3: If a live cell has more than 3 neighboring live cells it dies from overpopulation
                if a[x][y] == true && n_alive > 3 {
                    b[x][y] = false;
                }

                // Rule 4: If a dead cell has exactly 3 neighboring live cells it becomes live
                if a[x][y] == false && n_alive == 3 {
                    b[x][y] = true;
                }
            }
        }

        a = b;
        b = [[false; MAP_SIZE]; MAP_SIZE];
        thread::sleep(Duration::from_secs(1));
    }
}

fn init_map(a: &mut [[bool; MAP_SIZE]; MAP_SIZE]) {
    let mut rng = rand::thread_rng();

    for sub_arr in 0..MAP_SIZE {
        for element in 0..MAP_SIZE {
            a[sub_arr][element] = rng.gen_bool(P_TRUE);
        }
    }
}

fn print_screen(a: [[bool; MAP_SIZE]; MAP_SIZE]) {
    for sub_arr in a {
        for element in sub_arr {
            if element == true {
                print!("██");
            } else {
                print!("  ");
            }
        }
        print!("\n");
    }
    print!("\n");
}
