use rand::Rng;
use std::{thread, time::Duration};

const MAP_SIZE: usize = 15;
const T_STEP: u64 = 500;
const P_TRUE: f64 = 0.5;

fn main() {
    let mut rng = rand::thread_rng();

    loop {
        let mut a: [[bool; MAP_SIZE]; MAP_SIZE] = [[false; MAP_SIZE]; MAP_SIZE];

        init_map(&mut a, &mut rng);

        print_screen(a);
        thread::sleep(Duration::from_millis(T_STEP));
    }
}

fn init_map(a: &mut [[bool; 15]; 15], rng: &mut rand::prelude::ThreadRng) {
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
