use predicates::prelude::*;
fn read_file(path: &str) -> String {
        assert_ne!(path.len(), 0, "Path to file is invalid!");
        let content: String = std::fs::read_to_string(path).expect(&format!("file {path} not found"));
        return content;
    }
fn parse_content(content: &str) -> Vec<bool> {
    let mut vec: Vec<bool> = Vec::new();
    let lines: Vec<&str> = content.split("\r\n").collect();
    let level_predicate = predicate::function(|x: &i32| *x > 3 || *x < 1);
    for i in 0..lines.len(){
        let line = lines[i];
        let levels: Vec<&str> = line.split(" ").collect();
        let mut safe: bool = true;
        let mut asc = false;
        for j in 0..levels.len() - 1{
            let tmp1 = levels[j].parse::<i32>().unwrap();
            let tmp2: i32 = levels[j+1].parse::<i32>().unwrap();
            if j == 0{
                asc = tmp2 > tmp1;
            }
            let sub = (tmp1 - tmp2).abs();
            if level_predicate.eval(&sub) || (asc && tmp2 < tmp1 ) || (!asc && tmp1 < tmp2) {
                safe = false;
                break;
            }
        }
        vec.push(safe);
    }
    return vec;
}

fn main() {
    let day_input = "input.txt";
    let content = read_file(&day_input);
    println!("{content}");
    
    let levels_safe: Vec<bool> = parse_content(&content);
    let safe_levels= levels_safe.iter().filter(|&&x|x == true).count();
    println!("{:?}", levels_safe);
    println!("safe levels: {safe_levels}");
}
