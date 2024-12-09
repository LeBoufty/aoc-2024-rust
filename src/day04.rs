use std::error;
use crate::inputs::read_lines;
use crate::chargrid;

struct Word {
    x: Vec<(i32, i32)>,
    m: Vec<(i32, i32)>,
    a: Vec<(i32, i32)>,
    s: Vec<(i32, i32)>
}

fn parse_input(test: bool) -> Result<Word, Box<dyn error::Error>> {
    let lines = read_lines(4, test)?;
    let cg = chargrid::make_grid(&lines);
    let sortie = Word {
        x: cg.find_all('X'),
        m: cg.find_all('M'),
        a: cg.find_all('A'),
        s: cg.find_all('S')
    };
    Ok(sortie)
}

pub fn part1(test: bool) -> Result<u64, Box<dyn error::Error>> {
    let values = parse_input(test)?;
    let mut count = 0;
    for x in values.x {
        let possibles_m = vec![
            (-1, 0), (-1 ,1),
            (0, 1), (1, 1),
            (1, 0), (1, -1),
            (0, -1), (-1, -1)];
        let mut possibles_a: Vec<(i32, i32)> = Vec::new();
        for i in possibles_m {
            if values.m.contains(&(x.0 + i.0, x.1 + i.1)) {
                possibles_a.push((2*i.0, 2*i.1));
            }
        }
        if !possibles_a.is_empty() {
            let mut possibles_s: Vec<(i32, i32)> = Vec::new();
            for i in possibles_a {
                if values.a.contains(&(x.0 + i.0, x.1 + i.1)) {
                    possibles_s.push((3*i.0/2, 3*i.1/2));
                }
            }
            if !possibles_s.is_empty() {
                for i in possibles_s {
                    if values.s.contains(&(x.0 + i.0, x.1 + i.1)) {
                        count += 1;
                    }
                }
            }
        }
    }
    Ok(count)
}

pub fn part2(test: bool) -> Result<u64, Box<dyn error::Error>> {
    let values = parse_input(test)?;
    let mut count = 0;
    for a in values.a {
        let mut mas_count = 0;
        if values.m.contains(&(a.0 - 1, a.1 - 1))
            && values.s.contains(&(a.0 + 1, a.1 + 1)) {
                mas_count += 1;
            }
        if values.m.contains(&(a.0 + 1, a.1 + 1))
            && values.s.contains(&(a.0 - 1, a.1 - 1)) {
                mas_count += 1;
            }
        if values.m.contains(&(a.0 + 1, a.1 - 1))
            && values.s.contains(&(a.0 - 1, a.1 + 1)) {
                mas_count += 1;
            }
        if values.m.contains(&(a.0 - 1, a.1 + 1))
            && values.s.contains(&(a.0 + 1, a.1 - 1)) {
                mas_count += 1;
            }
        if mas_count == 2 {
            count += 1;
        }
    }
    Ok(count)
}

#[test]
fn test_part1() {
    assert_eq!(part1(true).unwrap(), 18);
}

#[test]
fn test_part2() {
    assert_eq!(part2(true).unwrap(),9);
}

