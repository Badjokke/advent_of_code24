use std::collections::HashMap;

mod io;

fn parse_file_content(file_content: &String) -> (Vec<i32>,Vec<i32>){
    let mut list1: Vec<i32> = Vec::new();
    let mut list2:Vec<i32> = Vec::new();
    
    let lines: Vec<&str> = file_content.split("\r\n").collect();
    for i in lines.iter() {
       let items: Vec<&str> = i.split("   ").collect();
        list1.push(items[0].parse::<i32>().unwrap());
        list2.push(items[1].parse::<i32>().unwrap());
    }
  
    return (list1, list2);
}

fn compute_list_diff(list1: &mut Vec<i32>, list2: &mut Vec<i32>) -> i32{
    list1.sort();
    list2.sort();   

    let mut diff: i32= 0;
    
    for i in 0..list1.len(){
        diff += (list2[i] - list1[i]).abs();
    } 

    return diff;
}

fn compute_list_similarity(list1: Vec<i32>, list2: Vec<i32>) -> i32{
    let mut freq_map: HashMap<i32, i32>= std::collections::HashMap::new();
    for i in 0..list2.len(){
        let val = freq_map.get(&list2[i]);
        if val.is_none(){
            freq_map.insert(list2[i], 1);
            continue;
        }
        freq_map.insert(list2[i], val.unwrap() + 1);
    }
    let mut similarity: i32 = 0;

    for i in 0..list1.len(){
        let item = freq_map.get(&list1[i]);
        let freq: &i32 = if item.is_none(){&0} else {item.unwrap()}; 
        similarity += list1[i] * (*freq);
    }
    return similarity;

}

fn main() {
    const X: &str = "input.txt";
    let file_content = io::filesystem::read_file(X);
    let mut two_lists = parse_file_content(&file_content);

    let diff: i32 = compute_list_diff(&mut two_lists.0, &mut two_lists.1);
    println!("list diff: {diff}");
    let similarity = compute_list_similarity(two_lists.0, two_lists.1);
    println!("list similarity: {similarity}")

}
