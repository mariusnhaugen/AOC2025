use std::io::{BufRead, BufReader, Lines};
use std::fs::File;


fn parse_line(s: String) -> u64{
    println!("{}", s);
    let chars: Vec<char> = s.chars().collect();
    let num_digits = 12;

    let mut digits = Vec::<u64>::new();
    let mut prev_index = 0;

    for n in 1..=num_digits {
        let (digit, index) = n_digit(n,num_digits ,prev_index, &chars);
        prev_index = index+1; //start index after previous hit
        digits.push(digit);
    }
    let number: u64  = digits.iter().fold(0, |acc,x| acc*10 + x );
    println!("parseline num: {}", number);
    number
}

fn n_digit(n:u8,num_digits: u8, prev_index:usize , chars: &Vec<char>) -> (u64, usize) {

    let max_char = chars.len() -(num_digits-n) as usize; //0 index, bound by n digits from the end
    println!("n_digit called with n: {}  prev_index: {}  max_char: {}/{}", n,prev_index, max_char,chars.len());
    let charslice = &chars[prev_index..max_char];

    let mut i = prev_index;    
    let mut digit = 0;
    let mut index = 0;
    
    for c in charslice {
        if digit == 9 {continue;};

            let nc = c.to_digit(10).expect("number");
            if nc > digit { 
                digit = nc; 
                index = i; 
                println!("c: {}  i: {}  - highest: {} at {}", c,i, digit, index);
            }
            i += 1;
    } 

    (digit as u64, index)
}

   

fn main() {
    // let input = "61111111171111119".to_string();
    let lines = read_file();

let mut total = 0;
    for line in lines{
      total += parse_line(line.unwrap());
    }

    println!("TOTAL: {}", total)
}

fn read_file() -> Lines<BufReader<File>>{
    const INPUT_PATH: &str = "input.txt";
    // const INPUT_PATH: &str = "example.txt";

    let file = File::open(INPUT_PATH).expect("File invalid");
    BufReader::new(file).lines()
}
