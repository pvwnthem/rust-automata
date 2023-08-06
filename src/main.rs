mod grid;

fn main() {
    let mut grid: grid::Grid = grid::Grid::new(50, 50);

    grid.populate_random();
    grid.display();
}
