use std::process;

fn main() {
    if let Err(e) = statistics::run() {
        println!("Application failed: {}", e);
        process::exit(1);
    }
}
