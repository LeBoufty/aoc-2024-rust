use std::error;
use crate::inputs::read_lines;
use std::collections::HashMap;
use crate::chargrid;

struct Antennas {
    height: i32,
    width: i32,
    antennas: HashMap<char, Vec<(i32, i32)>>
}

impl Antennas {
    fn get_antinodes(&self, repeat: bool) -> Vec<(i32, i32)> {
        let mut sortie: Vec<(i32, i32)> = Vec::new();
        for freq in self.antennas.keys() {
            for i1 in 0..self.antennas.get(freq).unwrap().len() {
                for i2 in 0..self.antennas.get(freq).unwrap().len() {
                    if i1 != i2 {
                        let pos1 = self.antennas.get(freq).unwrap()[i1];
                        let pos2 = self.antennas.get(freq).unwrap()[i2];
                        let mut antinode = (
                            2*pos2.0 - pos1.0,
                            2*pos2.1 - pos1.1
                        );
                        if !repeat {
                            if self.in_bounds(antinode) && !sortie.contains(&antinode) {
                                sortie.push(antinode);
                            }
                        } else {
                            while self.in_bounds(antinode) {
                                if !sortie.contains(&antinode) {
                                    sortie.push(antinode);
                                }
                                antinode.0 += pos2.0 - pos1.0;
                                antinode.1 += pos2.1 - pos1.1;
                            }
                        }
                    }
                }
            }
            if repeat && self.antennas.get(freq).unwrap().len() > 2 {
                for i in self.antennas.get(freq).unwrap() {
                    if !sortie.contains(&i) {
                        sortie.push(i.clone());
                    }
                }
            }
        }
        sortie
    }
    fn in_bounds(&self, coords: (i32, i32)) -> bool {
        return coords.0 >= 0 && coords.1 >= 0
            && coords.0 < self.height && coords.1 < self.width;
    }
}

fn parse_input(test:bool) -> Result<Antennas, Box<dyn error::Error>> {
    let lines = read_lines(8, test)?;
    let cg = chargrid::make_grid(&lines);
    let mut antennas: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
    for i in cg.find_all_symbols(Some('.')) {
        antennas.entry(i.get_symbol())
            .or_insert(Vec::new())
            .push(i.get_coords());
    }
    let sortie = Antennas {
        height: cg.height() as i32,
        width: cg.width() as i32,
        antennas
    };
    Ok(sortie)
}

pub fn part1(test: bool) -> Result<u32, Box<dyn error::Error>> {
    let values = parse_input(test)?;
    let sortie = values.get_antinodes(false).len() as u32;
    Ok(sortie)
}

pub fn part2(test: bool) -> Result<u32, Box<dyn error::Error>> {
    let values = parse_input(test)?;
    let sortie = values.get_antinodes(true).len() as u32;
    Ok(sortie)
}

#[test]
fn test_part1() {
    assert_eq!(part1(true).unwrap(), 14);
}

#[test]
fn test_part2() {
    assert_eq!(part2(true).unwrap(), 34);
}
