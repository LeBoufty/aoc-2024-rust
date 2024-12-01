mod day01;
mod inputs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Result : {}", day01::part2(false)?);
    Ok(())
}
