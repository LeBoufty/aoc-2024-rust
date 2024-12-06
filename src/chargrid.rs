#[derive(Clone)]
pub struct CharGrid {
    grid: Vec<Vec<char>>
}

impl CharGrid {
    pub fn height(&self) -> usize {
        return self.grid.len();
    }
    pub fn width(&self) -> usize {
        return self.grid[0].len();
    }
    pub fn is_rectangle(&self) -> bool {
        for i in self.grid.clone() {
            if i.len() != self.width() {
                return false;
            }
        }
        true
    }
    pub fn get(&self, l: usize, c: usize) -> Option<char> {
        if !self.is_in_grid(l, c) {return None}
        return Some(self.grid[l][c]);
    }
    pub fn set(&mut self, l:usize, c:usize, ch:char) {
        if !self.is_in_grid(l, c) {return}
        self.grid[l][c] = ch;
    }
    pub fn find_all(&self, ch: char) -> Vec<(i32, i32)> {
        let mut sortie: Vec<(i32, i32)> = Vec::new();
        for l in 0..self.height() {
            for c in 0..self.grid[l].len() {
                if self.get(l, c).unwrap() == ch {
                    sortie.push((l as i32, c as i32));
                }
            }
        }
        sortie
    }
    pub fn is_in_grid(&self, l:usize, c:usize) -> bool {
        return l < self.height() && c < self.width()
    }
}

pub fn make_grid(input: &Vec<String>) -> CharGrid {
    let mut grid: Vec<Vec<char>> = Vec::new();
    for i in input {
        grid.push(i.chars().collect());
    }
    CharGrid {grid}
}
