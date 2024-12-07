use std::time::{self};
mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod inputs;
mod chargrid;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let now = time::Instant::now();
    let result = day05::part2(false)?;
    println!("Result : {} // Time elapsed : {}ms", result, now.elapsed().as_millis());
    Ok(())
}
