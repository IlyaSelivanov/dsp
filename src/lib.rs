mod charts;
mod signal;

use crate::signal::*;
use std::error::Error;
use std::io;

const HELP: &str = "help";
const QUIT: &str = "quit";
const GENARATE: &str = "generate";
const MEAN: &str = "mean";
const MEDIAN: &str = "median";
const VARIANCE: &str = "variance";
const DEVIATION: &str = "deviation";

const DATA_LENGTH: usize = 1_00;

enum Command {
    Help,
    Quit,
    Genarate,
    Mean,
    Median,
    Variance,
    Deviation,
    None,
}

fn parse_command(cmd: &str) -> Command {
    match cmd {
        HELP => return Command::Help,
        QUIT => return Command::Quit,
        GENARATE => return Command::Genarate,
        MEAN => return Command::Mean,
        MEDIAN => return Command::Median,
        VARIANCE => return Command::Variance,
        DEVIATION => return Command::Deviation,
        _ => return Command::None,
    }
}

pub fn run() -> Result<(), Box<dyn Error>> {
    let mut signal = Signal::new();

    println!("Type 'help' to get list of available commands");

    loop {
        let mut input = String::new();

        println!("input: ");
        io::stdin().read_line(&mut input).expect("Invalid input.");

        let cmd = input.trim().to_lowercase();
        match parse_command(&cmd) {
            Command::Help => print_help(),
            Command::Quit => break,
            Command::Genarate => {
                let data = match Statistics::generate_random_vector(DATA_LENGTH, (0, 5)) {
                    Some(vec) => vec,
                    None => Vec::new(),
                };

                signal = Signal::from_vector(data);

                println!("Data successfully generated.");
            }
            Command::Mean => {
                if check_data_empty(&signal) {
                    continue;
                }

                println!("mean = {}", signal.statistics().mean());
            }
            Command::Median => {
                if check_data_empty(&signal) {
                    continue;
                }

                match signal.statistics().median() {
                    Median::Even(f, s) => println!("median = [{}, {}]", f, s),
                    Median::Odd(m) => println!("median = {m}"),
                    _ => continue,
                }
            }
            Command::Variance => {
                if check_data_empty(&signal) {
                    continue;
                }

                println!("variance = {}", signal.statistics().variance())
            }
            Command::Deviation => {
                if check_data_empty(&signal) {
                    continue;
                }

                println!("deviation = {}", signal.statistics().standard_deviation())
            }
            _ => continue,
        }
    }
    Ok(())
}

fn check_data_empty(signal: &Signal) -> bool {
    if signal.data().is_empty() {
        println!("Empty data. Use 'generate' or 'load' data first.");
        return true;
    }

    false
}

fn print_help() {
    println!("List of available commands:");
    println!("\t'help' - prints available commands;");
    println!("\t'generate' - generates random i64 vector;");
    println!("\t'mean' - calculates mean;");
    println!("\t'median' - calculates median;");
    println!("\t'variance' - calculates variance;");
    println!("\t'deviation' - calculates standard deviation;");
}
