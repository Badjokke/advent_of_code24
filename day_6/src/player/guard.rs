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
}
