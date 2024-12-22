use core::panic;
use std::{fmt::Debug, fs, str::FromStr};

pub fn read_file_contents(path: &str) -> String{
    assert_ne!(true, String::is_empty(&String::from(path)));
    let content_result = fs::read_to_string(path);
    let file_content = match content_result {
        Ok(file) => file,
        Err(err) => panic!("Failed to read file: {err:?}")
    };
    return file_content;
}

pub fn read_file_contents_to_lines(path: &str) -> Vec<String>{
    let content = read_file_contents(path);
    return Vec::from_iter(content.split("\n").map(String::from));
}

pub fn read_lines_as_bytes(path: &str) ->Vec<Vec<u8>>{
    let lines = read_file_contents_to_lines(path);
    let mut result:Vec<Vec<u8>> = Vec::new();
    for line in lines{
        result.push(Vec::from(line.as_bytes()));
    }
    return result;
}

pub fn read_lines_as_char(path: &str) ->Vec<Vec<char>>{
    let lines = read_file_contents_to_lines(path);
    let mut result:Vec<Vec<char>> = Vec::new();
    for line in lines{
        result.push(line.as_bytes().iter().map(|c| *c as char).collect());
    }
    return result;
}

pub fn read_lines_split_to<T>(path: &str, delimiter: &str) -> Vec<Vec<T>> where T: FromStr+Debug{
    assert_ne!(true, String::is_empty(&String::from(delimiter)));
    let lines = read_file_contents_to_lines(path);
    let mut result: Vec<Vec<T>> = Vec::new();
    for line in lines{
        let tmp:Vec<&str> = line.split(delimiter).collect();
        result.push(tmp.iter().map(|x| x.parse::<T>().unwrap_or_else(|_| panic!("Unable to parse!"))).collect());
    }
    return result;
}
