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
    list1.sort();
    list2.sort();

    return (list1, list2);
}

fn compute_list_diff(list1: Vec<i32>, list2: Vec<i32>) -> i32{
    let mut diff: i32= 0;
    
    for i in 0..list1.len(){
        diff += (list2[i] - list1[i]).abs();
    } 

    return diff;
}

fn main() {
    const X: &str = "../input.txt";
    let file_content = io::filesystem::read_file(X);
    let two_lists = parse_file_content(&file_content);
    let diff: i32 = compute_list_diff(two_lists.0, two_lists.1);
    println!("{diff}")
}
