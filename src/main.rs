mod day01;
mod day02;
mod inputs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Result : {}", day02::part2(false)?);
    Ok(())
}
