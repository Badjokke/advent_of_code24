use core::panic;
use std::collections::HashSet;
use crate::player::guard::Guard;


#[derive(Debug)]
pub struct Map{
    map: Vec<Vec<char>>,
    obstruction: char,
    visited_tiles: HashSet<String>,
    guard: Guard
}

impl Map{    
    
    pub fn new(map: Vec<Vec<char>>, guard: Guard)->Self{
        Map{map, guard, obstruction: '#', visited_tiles: HashSet::new()}
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
    
    fn is_out_of_map(&self, i: i32, j: i32) -> bool {
         (i < 0 || i == self.map.len() as i32) || (j < 0 || j == self.map.len() as i32)
    }
    

    fn is_obstruction(&self, i: usize, j: usize) -> bool {
        self.map[i][j] == self.obstruction
    }
    fn print_map(&self){
        for i in 0..self.map.len(){
            println!("{:?}",self.map[i]);
        }
    }


    pub fn simulate(&mut self) -> u32{
        let starting_position = self.find_guard_starting_position();
        let mut i = starting_position.0;
        let mut j = starting_position.1;
        let mut visited:u32 = 0;
        loop {
            if self.map[i][j] != 'X'{
                self.map[i][j] = 'X';
                visited += 1;
            }
            let mut tmp: (i32, i32) = self.guard.step(i as i32, j as i32);
            if self.is_out_of_map(tmp.0, tmp.1){
                break;
            }
            while self.is_obstruction(tmp.0 as usize,tmp.1 as usize){
                self.guard.change_direction();
                tmp = self.guard.step(i as i32, j as i32);
            }
            i = tmp.0 as usize;
            j = tmp.1 as usize;
        }
        self.print_map();
        visited
    }


}

