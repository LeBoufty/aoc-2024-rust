use std::error;
use crate::inputs::read_lines;

struct Word {
    x: Vec<(i32, i32)>,
    m: Vec<(i32, i32)>,
    a: Vec<(i32, i32)>,
    s: Vec<(i32, i32)>
}

fn parse_input(test: bool) -> Result<Word, Box<dyn error::Error>> {
    let lines = read_lines(4, test)?;
    let linelen = lines[0].len();
    let mut sortie = Word {
        x: Vec::new(),
        m: Vec::new(),
        a: Vec::new(),
        s: Vec::new()
    };
    for l in 0..lines.len() {
        for c in 0..linelen {
            match lines[l].chars().nth(c).unwrap() {
                'X' => sortie.x.push((l as i32, c as i32)),
                'M' => sortie.m.push((l as i32, c as i32)),
                'A' => sortie.a.push((l as i32, c as i32)),
                'S' => sortie.s.push((l as i32, c as i32)),
                _ => println!("Unknown char {}", lines[l].chars().nth(c).unwrap()),
            }
        }
    }
    Ok(sortie)
}

pub fn part1(test: bool) -> Result<u32, Box<dyn error::Error>> {
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

