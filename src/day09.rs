use core::error;
use crate::inputs::read_input;

fn parse_input(test:bool) -> Result<Vec<String>, Box<dyn error::Error>> {
    let data = read_input(9, test)?;
    let mut id = 0;
    let mut empty = false;
    let mut sortie: Vec<String> = Vec::new();
    for i in data.chars().map(|x| x.to_digit(10).unwrap() as usize) {
        if empty {
            sortie.append(&mut vec![".".to_string(); i]);
            empty = false;
        } else {
            sortie.append(&mut vec![id.to_string(); i]);
            id += 1;
            empty = true;
        }
    }
    Ok(sortie)
}

fn find_first_empty(val: &Vec<String>) -> usize {
    for i in 0..val.len() {
        if val.get(i).unwrap().eq(&".") {
            return i
        }
    }
    0
}

fn find_last_full(val: &Vec<String>) -> usize {
    for i in 1..=val.len() {
        if !val.get(val.len() - i).unwrap().eq(&".") {
            return val.len() - i
        }
    }
    0
}

fn get_value(val: &Vec<String>) -> u64 {
    let mut sortie = 0;
    for i in 0..(find_last_full(val)+1) as u64 {
        sortie += i * val[i as usize].parse::<u64>().unwrap() as u64;
    }
    sortie
}

pub fn part1(test: bool) -> Result<u64, Box<dyn error::Error>> {
    let mut values = parse_input(test)?;
    loop {
        let first_empty = find_first_empty(&values);
        let last_full = find_last_full(&values);
        if first_empty >= last_full {
            break;
        }
        values.swap(first_empty, last_full);
    }
    Ok(get_value(&values))
}

#[test]
fn test_part1() {
    assert_eq!(part1(true).unwrap(), 1928);
}