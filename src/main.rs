mod signal;

use std::collections::BTreeMap;

use crate::signal::*;

fn main() {
    let signal: Vec<i32> = match Statistics::generate_random_i32_vector(10, (10, 20)) {
        Some(vec) => vec,
        None => Vec::new(),
    };

    let btree_map_histogram = match build_btree_map_histogram(&signal) {
        Some(map) => map,
        _ => BTreeMap::new(),
    };

    // println!("{:?}", signal);
    println!("histogram:");
    println!("{:?}", btree_map_histogram);

    let signal = Signal::from_i32_vector(signal);
    signal.print_info();

    println!("{:?}", signal.data());
}
