use core::panic;
use std::collections::HashSet;
use crate::player::guard::Guard;


#[derive(Debug)]
pub struct Map{
    map: Vec<Vec<char>>,
    obstruction: char,
    free_space: char,
    visited_tiles: HashSet<String>,
    guard: Guard
}

impl Map{    
    
    pub fn new(map: Vec<Vec<char>>, guard: Guard)->Self{
        return Map{map, guard, obstruction: '#', free_space: '.', visited_tiles: HashSet::new()};
    }

    fn find_guard_starting_position(&self) -> (usize, usize){
        for i in 0..self.map.len(){
            for j in 0..self.map[i].len(){
                if self.map[i][j] == '^'{
                    return (i,j)
                }
            }
        }
        panic!("Guard facing upwards not found on map!")
    }
    
    pub fn simulate(&self) -> usize{
        let starting_position = self.find_guard_starting_position();
        println!("Guard starting position: {} {}", starting_position.0, starting_position.1);
        return 5;
    }


}

