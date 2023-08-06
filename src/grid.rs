use ansi_term::Colour::{Black, White};

#[derive(Debug)]
pub struct Cell {
    pub x: usize,
    pub y: usize,
    pub filled: bool,
}

impl Cell {
    pub fn new(x: usize, y: usize) -> Cell {
        Cell {
            x,
            y,
            filled: false,
        }
    }

    pub fn toggle(&mut self) {
        self.filled = !self.filled;
    }
}

#[derive(Debug)]
pub struct Grid {
    pub cells: Vec<Cell>,
    pub rows: usize,
    pub columns: usize,
}

impl Grid {
    pub fn new(rows: usize, columns: usize) -> Grid {
        let mut cells = Vec::new();
        for x in 0..rows {
            for y in 0..columns {
                cells.push(Cell::new(x, y));
            }
        }
        Grid {
            cells,
            rows,
            columns,
        }
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&Cell> {
        self.cells.get(x * self.columns + y)
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut Cell> {
        self.cells.get_mut(x * self.columns + y)
    }

    pub fn toggle(&mut self, cell: &Cell) -> Option<&Cell> {
        let (x, y) = (cell.x, cell.y);
        if let Some(cell) = self.get_mut(x, y) {
            cell.toggle();
            self.get(x, y)
        } else {
            None
        }
    }

    pub fn populate_random(&mut self) {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        for cell in self.cells.iter_mut() {
            cell.filled = rng.gen_bool(0.5);
        }
    }

    pub fn display(&self) {
        for row in 0..self.rows {
            for col in 0..self.columns {
                let cell = self.get(row, col).unwrap(); // Safe to unwrap since we're within the grid dimensions.

                let cell_display = if cell.filled {
                    Black.on(White).paint("  ").to_string()
                } else {
                    White.on(Black).paint("  ").to_string()
                };

                print!("{}", cell_display);
            }
            println!();
        }
    }
}
