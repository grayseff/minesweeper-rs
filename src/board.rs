// Module for the board and related structs and functions:
use rand::RngExt;
// List of neighbour indices, saves time later to declare now
const NEIGHBOURS: [(isize, isize); 8] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

pub struct Cell {
    pub mine: bool,
    pub revealed: bool,
    pub flagged: bool,
    pub adjacent: u8,
}
pub struct Board {
    pub width: usize,
    pub height: usize,
    cells: Vec<Cell>,
    initialised: bool,
}

impl Board {
    pub fn new(width: usize, height: usize, num_mines: u8) -> Self {
        let mut cells = Vec::with_capacity(width * height);
        for _ in 0..width * height {
            cells.push(Cell {
                mine: false,
                revealed: false,
                flagged: false,
                adjacent: 0,
            });
        }

        let mut board = Self {
            width,
            height,
            cells,
            initialised: false,
        };
        board.set_mines(num_mines);
        board
    }
    // get functions return the cell at (x,y)
    pub fn get(&self, x: usize, y: usize) -> &Cell {
        &self.cells[y * self.width + x]
    }
    fn get_mut(&mut self, x: usize, y: usize) -> &mut Cell {
        &mut self.cells[y * self.width + x]
    }
    // counts the number of adjacent mines for pos x,y
    fn count_neighbours(&self, x: usize, y: usize) -> u8 {
        let mut count = 0u8;

        for (dx, dy) in NEIGHBOURS {
            let nx = x as isize + dx;
            let ny = y as isize + dy;
            if nx >= 0 && ny >= 0 && nx < self.width as isize && ny < self.height as isize {
                if self.get(nx as usize, ny as usize).mine {
                    count += 1
                }
            }
        }
        count
    }
    // sets the adjacency of each cell
    pub fn set_adjacent(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let count = self.count_neighbours(x, y);
                self.get_mut(x, y).adjacent = count;
            }
        }
    }
    // function to place num_mines mines on the board
    fn set_mines(&mut self, num_mines: u8) {
        let mut rng = rand::rng();
        let mut placed = 0u8;
        assert!(num_mines as usize <= self.width * self.height);

        while placed < num_mines {
            let x = rng.random_range(0..self.width);
            let y = rng.random_range(0..self.height);
            if !self.get(x, y).mine {
                self.get_mut(x, y).mine = true;
                placed += 1
            }
        }
    }
    // function to guarantee that if the first click on fx,fy is a mine that we safe the cell
    pub fn ensure_safe_first_click(&mut self, fx: usize, fy: usize) {
        let mut rng = rand::rng();

        if self.get(fx, fy).mine {
            loop {
                let x = rng.random_range(0..self.width);
                let y = rng.random_range(0..self.height);
                if self.get(x, y).mine {
                } else {
                    self.get_mut(x, y).mine = true;
                    break;
                }
            }
            self.get_mut(fx, fy).mine = false;
        }
    }
// game logic behind revealing a cell
    pub fn reveal(&mut self, x: usize, y: usize) -> bool {
        if !self.initialised {
            self.ensure_safe_first_click(x, y);
            self.set_adjacent();
            self.initialised = true;
        }


        if self.get(x,y).revealed || self.get(x,y).flagged {
            return false;
        }
        self.get_mut(x,y).revealed = true;

        self.get(x,y).mine
    }

    pub fn has_won(&self) -> bool {
        for y in 0..self.height {
            for x in 0..self.width {
                let cell = self.get(x,y);
                    if !cell.mine && !cell.revealed {
                        return false;
                    } 
            }
        }
        true
    }
}
