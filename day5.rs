use std::fs::File;
use core::cmp::Ordering;
use core::ops::RangeInclusive;
use std::io::{BufReader,BufRead};


fn merge(ranges: &mut Vec<RangeInclusive<u64>>) {
    ranges.sort_by(|a, b| {
        match a.start().cmp(b.start()) {
            Ordering::Equal => a.end().cmp(b.end()),
            o => o
        }
    });

    let mut i = 0;
    while i < ranges.len() - 1 {
        let [a, b] = &mut ranges[i..=i+1] else { unreachable!() };

        if a.contains(b.end()) {
            ranges.remove(i + 1);
        } else if *b.start() <= (*a.end() + 1) {
            *a = *a.start()..=*b.end();
            ranges.remove(i + 1);
        } else {
            i += 1;
        }
    }
}

fn solve <I, S>(input: I) -> usize 
    where
        I: Iterator<Item = S>,
        S: AsRef<str>,
 {
    let mut ranges: Vec<RangeInclusive<u64>> = Vec::new();
let mut _part1 = 0;    
    let mut finish_list = false;
    for line in input {
        let line = line.as_ref().trim();
        if line.is_empty() {
            finish_list = true;
            merge(&mut ranges);
            continue;
        }

        if finish_list {
            let id = line.parse::<u64>().unwrap();
            if ranges.iter().any(|range| range.contains(&id)) {
                _part1 += 1;
            }
        } else {
            let (start, end) = line.split_once('-').unwrap();
            let (start, end) =  (start.parse::<u64>().unwrap(), end.parse::<u64>().unwrap());
            ranges.push(start..=end);
        }
    }

    let part2 = ranges.iter()
        .map(|r| (r.end() - r.start() + 1) as usize)
        .sum();

    part2
}

fn main() {
    let input = read_file().lines().map_while(Result::ok);
    let sol = solve(input);
    println!("{sol}");
}


fn read_file() -> BufReader<File> {
    const INPUT_PATH: &str = "input.txt";

    let file = File::open(INPUT_PATH).expect("File invalid");
    
    BufReader::new(file)

}


