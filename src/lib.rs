mod signal;
mod charts;

use std::error::Error;
use crate::signal::*;
use crate::charts::*;

pub fn run() -> Result<(), Box<dyn Error>> {
    let signal: Vec<i64> = match Statistics::generate_random_vector(1_000_000, (0, 10)) {
        Some(vec) => vec,
        None => Vec::new(),
    };

    let signal = Signal::from_vector(signal);
    signal.print_info();

    // println!("{:?}", signal.data());

    example_chart()?;

    Ok(())
}