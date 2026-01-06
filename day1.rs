use aoc::read_input_as_lines;

fn parse_line(mut s: String) -> i32 {
   let distance = s.split_off(1);
   let mut dist_num = distance.parse::<i32>().unwrap();
  
    if s == "L" {
        dist_num= -dist_num;
    }   

   dist_num 
}

fn solve(lines: Vec<String>) {
   
    let start = 50;
    let mut current_pos = start;
    let mut part1 = 0;
    let mut part2 = 0;
    let mut _skip_next_neg = false;

    for line in lines { 

        let change = parse_line(line);
        let mut new_pos = change + current_pos;
        let mut spins = 0;
        

        if new_pos > 99 {
            while new_pos > 99 {
                new_pos -= 100;
                spins += 1;
            }

        } else if new_pos < 0 {
            while new_pos < 0 {
                new_pos += 100;
                spins += 1;
            }
        }

        current_pos = new_pos;

        part1 += if current_pos == 0 { 1 } else { 0 };
        part2 += spins;
        
    }

   println!("Lands on zero: {} -  Part 2 Password {}", part1, part2);
}
   

fn main() {
    let input = read_input_as_lines();
    solve(input);
}
