use crate::grid;
use crate::ruleset;
pub struct Conways {
    pub name: String,
}

impl ruleset::Ruleset for Conways {
    fn new() -> Box<dyn ruleset::Ruleset + 'static> {
        Box::new(Conways { name: "Conway's Game of Life".to_string() })
    }
    

    fn next(&mut self, grid: &grid::Grid) -> grid::Grid {
        let mut next_grid: grid::Grid = grid::Grid::new(grid.rows, grid.columns);
        for x in 0..grid.rows {
            for y in 0..grid.columns {
                let cell = grid.get(x, y).unwrap();
                let mut neighbors_count = 0;
                for i in (x.saturating_sub(1))..=(x.saturating_add(1)) {
                    for j in (y.saturating_sub(1))..=(y.saturating_add(1)) {
                        if let Some(neighbor) = grid.get(i, j) {
                            if neighbor.filled && !(i == x && j == y) {
                                neighbors_count += 1;
                            }
                        }
                    }
                }
    
                if cell.filled {
                    if neighbors_count == 2 || neighbors_count == 3 {
                        next_grid.toggle(cell, true);
                    } else {
                        next_grid.toggle(cell, false);
                    }
                } else {
                    if neighbors_count == 3 {
                        next_grid.toggle(cell, true);
                    } else {
                        next_grid.toggle(cell, false);
                    }
                }
            }
        }
        next_grid
    }
    
}
