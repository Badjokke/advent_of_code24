use shared_lib::io::filesystem;
mod map;
mod player;
use map::map::Map;
use player::guard::Guard;


fn main() {
    let result = filesystem::read_lines_as_char("input.txt");
    let mut map = Map::new(result, Guard::new());
    let visited = map.simulate();
    println!("Visited {visited} positions");
}
