use crate::grid;
use crate::ruleset;

pub struct Automata<'a> {
    pub grid: grid::Grid,
    pub ruleset: Box<dyn ruleset::Ruleset + 'a>,
}

impl<'a> Automata<'a> {
    pub fn new(grid: grid::Grid, ruleset: Box<dyn ruleset::Ruleset + 'a>) -> Automata<'a> {
        Automata {
            grid,
            ruleset,
        }
    }

    pub fn display(&self) {
        self.grid.display();
    }

    pub fn next(&mut self) {
        let next_grid: grid::Grid = self.ruleset.next(&self.grid);
        self.grid = next_grid;
    }
}