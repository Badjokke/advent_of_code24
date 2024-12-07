use std::{fs::File, io::{BufReader, Read}};
use regex::Regex;

fn read_input_file(path: &str) -> String{
    let f = File::open(path).expect("File not found!");
    let mut reader =  BufReader::new(f);
    let mut buffer: String = String::new();
    reader.read_to_string(&mut buffer).unwrap();
    return buffer;
}

fn find_mul_patterns(haystack: &String) -> Vec<&str>{
    let re = Regex::new(r"mul\((\+|\-)?\d+\,(\+|\-)?\d+\)").unwrap();
    let vec: Vec<&str> = re.find_iter(&haystack).map(|collect| collect.as_str()).collect();
    return vec;
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

fn main() {
    let input_file = read_input_file("input.txt");
    let vec = find_mul_patterns(&input_file);
    let result: i32= add_multiplications(vec);
    println!("{result}");
}
