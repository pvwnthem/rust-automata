use crate::grid;

pub trait Ruleset {
    fn new() -> Box<dyn Ruleset> where Self: Sized;
    fn next(&mut self, grid: &grid::Grid) -> grid::Grid;   
}