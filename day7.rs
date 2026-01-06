use std::collections::HashMap;

use aoc::read_input_as_2d_vec;

fn solve(input: Vec<Vec<String>>) -> u32 {
    let mut split_count = 0;
    let mut lasers = HashMap::new();
    for i in 0..input[0].len(){
        if input[0][i] == "S"{
            lasers.insert(i, true);
            break;
        }
    }

    for line in input {
        for (index, laser_exists) in lasers.clone() {
            if line[index] == "^" && laser_exists {
                split_count += 1;
                lasers.insert(index+1, true);
                lasers.insert(index-1, true);
                lasers.insert(index, false);
            }
        }
    }

    split_count
}

fn solve_2(input: Vec<Vec<String>>) -> u64 {
    let mut lasers = HashMap::<usize, u64>::new();
    let mut numlines = 0;
    
    for i in 0..input[0].len(){
        if input[0][i] == "S"{
            lasers.insert(i, 1);
            break;
        }
    }

    for line in input {
        for (index, num_lasers) in lasers.clone() {
            if line[index] == "^" && num_lasers>0 {
                    let num = num_lasers.clone();
                    *lasers.entry(index+1).or_insert(0) += num;
                    *lasers.entry(index-1).or_insert(0) += num;
                    *lasers.entry(index).or_insert(0) -= num;
            }
        }
        numlines += 1;
        let laseramount:u64 = lasers.values().sum();
        println!("[{}] lasers: {}", numlines, laseramount);
    }
    lasers.values().sum()
}

fn main (){
    let input = read_input_as_2d_vec();
    let input_2 = input.clone();

    let part1 = solve(input);
    println!("Part 1: {}", part1);

    let part2 = solve_2(input_2);
    println!("Part 2: {}", part2);
}
