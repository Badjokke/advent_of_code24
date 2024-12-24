#[derive(Debug)]
enum Direction{
    Up, Down, Left, Right
}

#[derive(Debug)]
pub struct Guard{
    direction: Direction,
}

impl Guard{
    pub fn new() -> Self{
        Guard{direction: Direction::Up}
    }

    pub fn change_direction(&mut self) {
        self.direction = match self.direction {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up
        }    
    }
    pub fn step(&self, i: i32, j: i32) -> (i32, i32){
        match self.direction{
            Direction::Up => (i - 1, j),
            Direction::Down => (i + 1, j),
            Direction::Left => (i, j - 1),
            Direction::Right => (i, j + 1)
        }
    }

}
