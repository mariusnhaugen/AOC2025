use std::fs::File;
use std::io::{BufReader,BufRead};
use std::cmp;


struct Range {
    start: u64,
    end: u64 
}

fn main() {
    let (mut ranges, ids) = read_file();
    let count = count_fresh_ids(&ranges, ids);
    let rcount = count_fresh_ranges(&mut ranges);

    println!("End count: {}   {}", count, rcount)
}


fn count_fresh_ids(ranges: &Vec<Range>, ids: Vec<u64>) -> u32 {
  
    let mut count = 0;
    for &id in &ids {
        for range in ranges {
            if id >= range.start && id <= range.end {
                count += 1;
                break;
            } 
        }
    }

    count
}

fn count_fresh_ranges(ranges: &mut Vec<Range>) -> u64 {
    ranges.sort_by(|a,b| a.start.cmp(&b.start));

    //collapse ranges with duplicate members
    let mut i = 0;
    while i < ranges.len()-1 {
        if ranges[i].end > ranges[i+1].start {
            let rstart = cmp::min(ranges[i].start, ranges[i+1].start);
            let rend = cmp::max(ranges[i].end, ranges[i+1].end);
            let r = Range {start: rstart ,end: rend };
           
            ranges[i] = r;
            ranges.remove(i+1);
        }
        i+=1;

    }

    let count = ranges.iter().map(|r| r.end - r.start + 1 ).sum();
    count
}

fn read_file() -> (Vec<Range>, Vec<u64>) {
    const INPUT_PATH: &str = "input.txt";

    let file = File::open(INPUT_PATH).expect("File invalid");
    let mut lines = BufReader::new(file).lines();
   
    let mut ingredient_switch = false;
    let mut ranges: Vec<Range> = Vec::new();
    let mut ingredient_ids: Vec<u64> = Vec::new();
  
    while !ingredient_switch {
        let line = lines.next().unwrap().expect("string");
        if line == "".to_string() { 
            ingredient_switch = true; 
            continue;
        }
        let ab: Vec<&str> = line.split("-").collect();
        let a = ab[0].parse::<u64>().expect("Number");
        let b = ab[1].parse::<u64>().expect("Number");
        ranges.push(Range {start:a , end:b} );
    }
    if INPUT_PATH == "input.txt"{
    while let Some(line) = lines.next() {
        let line = line.unwrap();

        let id = line.parse::<u64>().expect("Number");
        ingredient_ids.push(id);
    }
    }

    (ranges, ingredient_ids)

}


