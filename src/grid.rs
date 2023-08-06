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

    pub fn display(&self) {
        let max_width = self.cells.iter().map(|cell| cell.x.to_string().len()).max().unwrap_or(0);
        let max_height = self.cells.iter().map(|cell| cell.y.to_string().len()).max().unwrap_or(0);

        for x in 0..self.rows {
            for y in 0..self.columns {
                if let Some(cell) = self.get(x, y) {
                    let x_padding = " ".repeat(max_width - cell.x.to_string().len());
                    let y_padding = " ".repeat(max_height - cell.y.to_string().len());
                    if cell.filled {
                        print!("{}", Black.on(White).paint(format!("{}{} ", x_padding, " ")).to_string());
                    } else {
                        print!("{}", White.on(Black).paint(format!("{}{} ", y_padding, " ")).to_string());
                    }
                }
            }
            println!();
        }
    }
}
