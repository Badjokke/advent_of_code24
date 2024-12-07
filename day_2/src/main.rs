fn read_file(path: &str) -> String {
        assert_ne!(path.len(), 0, "Path to file is invalid!");
        let content: String = std::fs::read_to_string(path).expect(&format!("file {path} not found"));
        return content;
    }

fn test_level_diff(desc:&bool, tmp1: &i32, tmp2: &i32 ) -> bool{
    return (*desc && tmp2 > tmp1 ) || (!*desc && tmp1 > tmp2)
}
fn create_level_diff_predicate() -> impl Fn(&i32) -> bool {
    move |x: &i32|{
        return *x > 3 || *x < 1;
    }
}

fn is_valid(levels: &[i32]) -> bool{
    if levels.len() < 2 {
        return true; // A single level or empty is trivially safe
    }
    let desc = levels[0] > levels[1];

    let predicate = create_level_diff_predicate();
    for i in 0..levels.len() - 1 {
        let diff = (levels[i + 1] - levels[i]).abs();
        let diff_too_high = predicate(&diff);
        let wrong_order = test_level_diff(&desc, &levels[i], &levels[i+1]);
        if diff_too_high || wrong_order {
            return false;
        }
    }
    true
}


fn is_level_safe(levels: Vec<&str>)->bool{
    let level_values: Vec<i32> = levels.iter().map(|x| x.parse::<i32>().unwrap()).collect();

    for i in 0..level_values.len() {
        let mut modified_levels = level_values.clone();
        modified_levels.remove(i);
        if is_valid(&modified_levels) {
            return true;
        }
    }
    return false
}


fn parse_content(content: &str) -> Vec<bool> {
    let mut vec: Vec<bool> = Vec::new();
    let lines: Vec<&str> = content.split("\r\n").collect();
    for i in 0..lines.len(){
        let line = lines[i];
        let levels: Vec<&str> = line.split(" ").collect();
        vec.push(is_level_safe(levels));
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
