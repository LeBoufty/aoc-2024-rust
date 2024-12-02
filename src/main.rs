use std::time::{self, Duration, Instant};
mod day01;
mod day02;
mod inputs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let now = time::Instant::now();
    let result = day02::part2(false)?;
    println!("Result : {} // Time elapsed : {}µs", result, now.elapsed().as_micros());
    Ok(())
}
