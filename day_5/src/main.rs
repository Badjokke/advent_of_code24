use std::{collections::HashSet, vec};

use regex::Regex;
fn read_input(path: &str) -> String{
    return std::fs::read_to_string(path).expect("File not found")
}
fn iter_to_set(data: &[u32]) -> std::collections::HashSet<u32> {
    return HashSet::from_iter(data.iter().cloned());
}
fn parse_dependency(item: &str) -> (u32,u32){
    let tmp: Vec<&str> = item.split("|").collect();
    return (tmp[1].parse::<u32>().unwrap(), tmp[0].parse::<u32>().unwrap());
}
fn parse_order_line(item: &str) -> Vec<u32>{
    let tmp: Vec<&str> = item.split(",").collect();
    return tmp.iter().map(|x| (x).parse::<u32>().unwrap()).collect();
}
fn parse_input(content: &str) -> (std::collections::HashMap<u32, Vec<u32>>, Vec<Vec<u32>>){
    let lines: Vec<&str> = content.split("\r\n").collect();
    let mut order: Vec<Vec<u32>> = Vec::new();
    let mut dependency: std::collections::HashMap<u32, Vec<u32>> = std::collections::HashMap::new();
    let re = Regex::new(r"\d+\|\d+").unwrap();
    for line in lines{
        if !re.is_match(line){
            let tmp = parse_order_line(line);
            order.push(tmp);
            continue;
        }
            let tmp = parse_dependency(line);
            match dependency.get_mut(&tmp.0){
                Some(list) => {list.push(tmp.1);}
                None => {dependency.insert(tmp.0, vec![tmp.1]);}
            }
    }

    return (dependency, order);

}


fn level_dependencies_valid(level: &Vec<u32>, dependencies: & std::collections::HashMap<u32,Vec<u32>>) -> bool{
    println!("{:?}",level);
    let mut printed_pages: HashSet<u32> = HashSet::new();
    let pages_in_level = iter_to_set(level);
    for i in  0..level.len(){
        let level = level[i];
        match dependencies.get(&level){
            Some(level_dependencies) => {
                for dependency in level_dependencies{
                    if pages_in_level.contains(dependency) && !printed_pages.contains(dependency){
                        return false;
                    }
                }
            }
            None =>{printed_pages.insert(level);}
        }
        printed_pages.insert(level);
    }
    return true;
}

fn filter_valid_levels(levels: Vec<Vec<u32>>, dependencies: &std::collections::HashMap<u32, Vec<u32>>) -> (Vec<Vec<u32>>,Vec<Vec<u32>>) {
    let mut valid_levels: Vec<Vec<u32>> = Vec::new();
    let mut invalid_levels: Vec<Vec<u32>> = Vec::new();
    for level in levels{
        if level_dependencies_valid(&level, &dependencies){
            valid_levels.push(level);
            continue;
        }
        invalid_levels.push(level);
    }
    return (valid_levels, invalid_levels);
}

fn sum_valid_level_mid_values(levels: &Vec<Vec<u32>>) -> u32{
    let mut sum = 0;
    for level in levels{
        sum += level[level.len()/2];
    }
    return sum;
}


fn extract_all_dependencies(item: &u32, dependencies: &std::collections::HashMap<u32, Vec<u32>>, visited: &mut HashSet<u32>, unique_items: &HashSet<u32>) -> Vec<u32>{
    let mut vec: Vec<u32> =Vec::new();
    match dependencies.get(item){
        Some(deps) =>{
            for tmp in deps{
                if !unique_items.contains(tmp) || visited.contains(tmp){
                    continue;
                }
                let mut foo = extract_all_dependencies(tmp, dependencies, visited, unique_items);
                vec.append(&mut foo);
            }
        }
        None => {
            visited.insert(*item);
        }
    }
    vec.push(*item);    
    visited.insert(*item);
    return vec;
}


fn reoder_level(level: &Vec<u32>, dependencies: &std::collections::HashMap<u32, Vec<u32>>) -> Vec<u32>{
    let mut visited: HashSet<u32> = HashSet::new();
    let mut reordered_level: Vec<u32> = Vec::new();
    let unique_items = iter_to_set(level);
    for item in level{
        if visited.contains(item){
            continue;
        }
        let mut tmp = extract_all_dependencies(item, dependencies, &mut visited, &unique_items);
        reordered_level.append(&mut tmp);
        println!("{:?}", reordered_level);
        println!("{:?}", visited);


    }
    return reordered_level;
}


fn order_incorrect_levels(levels: &Vec<Vec<u32>>, dependencies: &std::collections::HashMap<u32, Vec<u32>>) -> Vec<Vec<u32>>{
    let mut reordered:Vec<Vec<u32>> = Vec::new();
    for level in levels{
        let tmp = reoder_level(level, dependencies);
        reordered.push(tmp);
    }
    return reordered;
}


fn main() {
    let path = "input.txt";
    let content = read_input(path);
    let foo = parse_input(&content);
    let levels = filter_valid_levels(foo.1, &foo.0);
    let valid_levels = levels.0;
    let invalid_levels = levels.1;
    println!("valid levels: {:?}", valid_levels);
    println!("invalid levels: {:?}", invalid_levels);
    let mid_sum_of_valid_levels = sum_valid_level_mid_values(&valid_levels);
    println!("sum: {:?}", mid_sum_of_valid_levels);
    let reordered_levels = order_incorrect_levels(&invalid_levels, &foo.0);
    println!("reodered: {:?}", reordered_levels);
    let mid_sum_of_invalid_levels = sum_valid_level_mid_values(&reordered_levels);
    println!("sum reodered: {:?}", mid_sum_of_invalid_levels);

}
