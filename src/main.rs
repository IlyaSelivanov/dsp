mod signal;

use crate::signal::*;

fn main() {
    let signal: Vec<i32> = match Statistics::generate_random_i32_vector(10, (10, 20)) {
        Some(vec) => vec,
        None => Vec::new(),
    };

    let signal = Signal::from_i32_vector(signal);
    signal.print_info();

    // println!("{:?}", signal.data());
}
