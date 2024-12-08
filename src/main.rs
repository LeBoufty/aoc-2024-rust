use std::env;
use std::time::{self};
use functions::FUNCTIONS;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod inputs;
mod chargrid;
mod functions;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 4 {
        eprintln!("Usage: {} <day> <part> <test>", args[0]);
        std::process::exit(1);
    }

    let day = &args[1];
    let part = &args[2];
    let test = args[3].parse::<bool>().unwrap_or(false);

    let now = time::Instant::now();
    let result = match FUNCTIONS.get(&(day.as_str(), part.as_str())) {
        Some(func) => func(test)?,
        None => {
            eprintln!("Invalid day or part");
            std::process::exit(1);
        }
    };

    println!("Result : {} // Time elapsed : {}ms", result, now.elapsed().as_millis());
    Ok(())
}