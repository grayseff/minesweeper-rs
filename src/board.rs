// Module for the board and related structs and functions:
//
// List of neighbour indices, saves time later to declare now
const NEIGHBOURS: [(isize, isize); 8] = [
    (-1, -1),
    ( 0, -1),
    ( 1, -1),
    (-1,  0),
    ( 1,  0),
    (-1,  1),
    ( 0,  1),
    ( 1,  1),
];

pub struct Cell {
    mine: bool,
    revealed: bool,
    flagged: bool,
    adjacent: u8,
}
pub struct Board {
    width: usize,
    height: usize,
    cells:Vec<Cell>,
}


impl Board {
    pub fn new(width: usize, height: usize) -> Self {
        let mut cells = Vec::with_capacity(width*height);
        for _ in 0..width*height {
            cells.push(Cell {
                mine: false,
                revealed: false,
                flagged: false,
                adjacent: 0,
            });
        }
        Self{
            width , 
            height , 
            cells,
        }   

        
    }
// get functions return the cell at (x,y)
    fn get(&self, x: usize , y: usize) -> &Cell {
        &self.cells[y*self.width +x]
    }
    fn get_mut(&mut self, x :usize, y: usize) -> &mut Cell {
        &mut self.cells[y*self.width + x]
    }
// counts the number of adjacent mines for pos x,y
    fn count_neighbours(&self , x: usize , y: usize) -> u8{
        let mut count = 0u8 ;

        for (dx , dy) in NEIGHBOURS {
            let nx = x as isize + dx;
            let ny = y as isize + dy;
            if nx >= 0
                && ny >= 0 
                && nx < self.width as isize 
                && ny < self.height as isize {
                if self.get(nx as usize, ny as usize).mine {
                    count += 1
                }
            }
        }
        count
    }
// sets the adjacency of each cell 
    fn set_adjacent(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let count = self.count_neighbours(x,y);
                self.get_mut(x,y).adjacent = count;
            }
        }
    }
}
