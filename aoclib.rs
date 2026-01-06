use std::fs::{File, read_to_string};
use std::io::{BufReader,BufRead};

const INPUT_PATH: &str = "input.txt";
const EXAMPLE_PATH: &str = "example.txt";

pub fn read_input_as_lines() -> Vec<String> {
    let f = File::open(INPUT_PATH).expect("File invalid");
    BufReader::new(f).lines().map_while(Result::ok).collect()
} 

pub fn read_input_as_string() -> String {
    read_to_string(INPUT_PATH).expect("File invalid")
} 


pub fn read_example_as_lines() -> Vec<String> {
    let f = File::open(EXAMPLE_PATH).expect("File invalid");
    BufReader::new(f).lines().map_while(Result::ok).collect()
} 

pub fn read_example_as_string() -> String {
    read_to_string(EXAMPLE_PATH).expect("File invalid")
} 

pub fn read_input_as_2d_vec() -> Vec<Vec<String>> {
    let lines = read_input_as_lines();
    split_lines_chars(lines)
} 



pub fn split_lines_whitespace(lines: &Vec<String>) -> Vec<Vec<String>> {
    let mut chars: Vec<Vec<String>> = Vec::new();
    
    for l in lines {
        let x = l.split_whitespace().map(String::from).collect();
        chars.push(x);
    }
    chars
}

pub fn split_lines_chars(lines: Vec<String>) -> Vec<Vec<String>> {
    let mut chars: Vec<Vec<String>> = Vec::new();
    for l in lines {
        let x = l.chars().map(String::from).collect();
        chars.push(x);
    }
    chars
}

