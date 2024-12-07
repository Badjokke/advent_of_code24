use std::{fs::File, io::{BufReader, Read}};
use regex::Regex;

fn read_input_file(path: &str) -> String{
    let f = File::open(path).expect("File not found!");
    let mut reader =  BufReader::new(f);
    let mut buffer: String = String::new();
    reader.read_to_string(&mut buffer).unwrap();
    return buffer;
}

fn find_mul_patterns(haystack: &String) -> (Vec<(usize, &str)>,Vec<(usize, &str)>) {
    let re = Regex::new(r"mul\((\+|\-)?\d+\,(\+|\-)?\d+\)").unwrap();
    let do_dont_regex = Regex::new(r"don't\(\)|do\(\)").expect("Failed to compile regex pattern");
    let vec: Vec<(usize, &str)> = re.find_iter(&haystack).map(|collect| (collect.start(), collect.as_str())).collect();
    let do_dont_vec: Vec<(usize, &str)> = do_dont_regex.find_iter(&haystack).map(|collect| (collect.start(), collect.as_str())).collect();
    return (vec, do_dont_vec);
}

fn parse_expression(mul_expression: &str) -> (i32, i32){
    let split_vector: Vec<&str> = mul_expression.split(",").collect();
    let first_digit = split_vector[0].get(4..).unwrap().parse::<i32>().expect("Failed to parse");
    let second_digit  = split_vector[1].get(0..split_vector[1].len()-1).unwrap().parse::<i32>().expect("Failed to parse");
    return (first_digit, second_digit)

}

fn add_multiplications(vec:Vec<&str>)->i32{
    let mut sum: i32 = 0;
    for multiplication_expression in vec{
        let expression = parse_expression(multiplication_expression);
        sum += expression.0 * expression.1;
    }
    return sum;
}

fn create_ranges(driving_vec: Vec<(usize, &str)>) -> Vec<usize>{
    let mut ranges = Vec::new();
    let mut enabled = true;
    for i in 0..driving_vec.len(){
        let item = driving_vec[i];
        if enabled{
            if item.1 == "do()"{
                continue;
            }
            enabled = false;
            ranges.push(item.0);
        }

        if item.1 == "don't()"{
            continue;
        }
        enabled = true;
        ranges.push(item.0);
    }

    return ranges;
}


fn add_multiplication_part2(multiplication_vec: Vec<(usize, &str)>, driving_vec: Vec<(usize, &str)>) -> i32{
    let mut enabled = true;
    let mut current_range = 0;
    let mut sum = 0;
    let ranges = create_ranges(driving_vec);
    
    for i in 0..multiplication_vec.len(){
        let item = multiplication_vec[i];
        if  current_range < ranges.len() && item.0 >= ranges[current_range] {
            enabled = !enabled;
            current_range += 1;
        }
        if !enabled{
            continue;
        }
        let digits = parse_expression(item.1);
        sum += digits.0 * digits.1;
    }
    return sum;
}


fn main() {
    let input_file = read_input_file("input.txt");
    let vec = find_mul_patterns(&input_file);
    let result: i32= add_multiplication_part2(vec.0, vec.1);
    println!("{result}");
}
