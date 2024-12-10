use crate::inputs::read_lines;

struct Template {

}

fn parse_input(test:bool) -> Result<Template, Box<dyn error::Error>> {
    let lines = read_lines(5, test)?;
    let sortie: Template;
    Ok(sortie)
}

pub fn part1(test: bool) -> Result<u64, Box<dyn error::Error>> {
    let values = parse_input(test)?;
    Ok(0)
}

pub fn part2(test: bool) -> Result<u64, Box<dyn error::Error>> {
    let values = parse_input(test)?;
    Ok(0)
}

#[test]
fn test_part1() {
    assert_eq!(part1(true).unwrap(), 0);
}

#[test]
fn test_part2() {
    assert_eq!(part2(true).unwrap(), 0);
}