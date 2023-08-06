mod grid;
mod automata;
mod ruleset;
mod conways;

fn main() {
    <conways::Conways as ruleset::Ruleset>::new();
    let mut grid: grid::Grid = grid::Grid::new(50, 50);
    grid.populate_random();

    let mut auto: automata::Automata<'_> = automata::Automata::new(grid, <conways::Conways as ruleset::Ruleset>::new());


    loop {
        auto.display();
        // sleep for 1 second
        std::thread::sleep(std::time::Duration::from_millis(100));
        // clear the screen
        
        auto.next();
    }
    
    
}
