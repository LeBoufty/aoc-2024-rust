use std::time::{self};
mod day01;
mod day02;
mod day03;
mod day04;
mod inputs;
mod chargrid;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let now = time::Instant::now();
    let result = day04::part2(false)?;
    println!("Result : {} // Time elapsed : {}µs", result, now.elapsed().as_micros());
    Ok(())
}
