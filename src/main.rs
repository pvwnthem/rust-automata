mod grid;
fn main() {
    let mut grid = grid::Grid::new(50, 50);
    grid.toggle(&grid::Cell::new(0, 0));
    grid.display();
}
