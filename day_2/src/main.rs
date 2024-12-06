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

fn is_level_bad(level1: &i32, level2: &i32, desc: &bool) -> bool{
    let predicate = create_level_diff_predicate();
    let diff = (level2 - level1).abs();
    let diff_too_high = predicate(&diff);
    let wrong_order = test_level_diff(desc, level1, level2);
    return diff_too_high ||  wrong_order;
}

fn level_str_to_i32(level: &str) -> i32{
    return level.parse::<i32>().unwrap();
}

fn is_level_safe(levels: Vec<&str>)->bool{
    let mut safe: bool = true;
    let mut removed: i32 = -1;
    let mut desc = false;
    for j in 0..levels.len() - 1{
        if j as i32 == removed {
            continue;
        }

        let level1 = level_str_to_i32(levels[j]);
        let level2 = level_str_to_i32(levels[j+1]);
        if j == 0 {
            desc = level1 > level2;
        }
        if is_level_bad(&level1, &level2, &desc){
            safe = false;
            if removed != -1 {
                break;
            }
            if (j+2) == levels.len(){
                safe = true;
                break;
            }
            if j == 0 {
                removed = 0;
                safe = true;
                //desc = level2 > level_str_to_i32(levels[2]);
                continue;
            }

            let previous_level = level_str_to_i32(levels[j-1]);
            if !is_level_bad(&previous_level, &level2, &desc){
                removed = j as i32;
                safe = true;
                continue;
            }
            let next_level = level_str_to_i32(levels[j+2]);
            if !is_level_bad(&level1, &next_level, &desc){
                removed = (j+1) as i32;
                safe = true;
                continue;
            }

        }
    }
    return safe;

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
