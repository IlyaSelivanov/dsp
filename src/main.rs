mod signal;

use crate::signal::*;

fn main() {
    let signal: Vec<i64> = match Statistics::generate_random_vector(1000, (0, 1)) {
        Some(vec) => vec,
        None => Vec::new(),
    };

    let signal = Signal::from_vector(signal);
    signal.print_info();

    // println!("{:?}", signal.data());
}
