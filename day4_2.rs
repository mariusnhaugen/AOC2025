use std::io::{BufRead, BufReader};
use std::fs::File;


fn parse_lines(mut lines: Vec<Vec<char>>, recursive: bool) {
  
    let mut count = 0;
    
    if recursive == false {
        for i in 1..lines.len()-1 { //1 and -1 to take into account the padded lines
            let (above, rest) = lines.split_at_mut(i);
            let (target, below) = rest.split_at_mut(1);
        
            count += check_line(&above[above.len()-1], &mut target[0], &below[0]);
        }
    } else {
        let mut inner_count = 1 ;
        while inner_count > 0 {
            inner_count = 0;
            for i in 1..lines.len()-1 { //1 and -1 to take into account the padded lines
                let (above, rest) = lines.split_at_mut(i);
                let (target, below) = rest.split_at_mut(1);
            
                inner_count += check_line(&above[above.len()-1], &mut target[0], &below[0]);
            }

            println!("-------------------------- {}", inner_count);
            clean_lines(&mut lines);
            count += inner_count;
        }

    }
    println!("Whole diagram has {} valid rolls", count);
}


fn clean_lines(lines: &mut Vec<Vec<char>>) {

    for line in lines {
        for c in line {
            if *c == 'x'{
               *c = '.';
            }
        }
    }
}


fn check_line(p: &Vec<char>, t: &mut Vec<char>, n: &Vec<char>) -> u32 {
    let mut count = 0;
    println!(": {}", p.iter().fold("".to_string(),|acc,x| format!("{}{}",acc.to_string(), x.to_string())));
    println!(": {}", t.iter().fold("".to_string(),|acc,x| format!("{}{}",acc.to_string(), x.to_string())));
    println!(": {}", n.iter().fold("".to_string(),|acc,x| format!("{}{}",acc.to_string(), x.to_string())));

    for i in 0..t.len() {
        if t[i] == '@' {
            let mut ats = 0;
            //Up and left checks x
            //up
            if p.len() > 1 {
                if i > 0 && (p[i-1]=='@' || p[i-1]=='x' ) { ats += 1 } 
                if p[i]=='@' || p[i]=='x' { ats += 1 } 
                if i+1 < t.len() && (p[i+1]=='@' || p[i+1]=='x') { ats += 1}
            }
            //left,right
            if i > 0 && (t[i-1]=='x'|| t[i-1]=='@' ) { ats += 1 } 
            if i+1 < t.len() && t[i+1]=='@' { ats += 1}
          
            //down i > 0 
            if n.len() > 1 {
                if i>0 && n[i-1]=='@' { ats += 1 } 
                if n[i]=='@' { ats += 1 } 
                if i+1 < t.len() && n[i+1]=='@' { ats += 1}
            }
            
            if ats < 4 { 
                count += 1;
                t[i] = 'x'; 
            }
       }
    }
    println!("Line has {} valid rolls", count);
    println!("");

    count
} 


fn main() {
    let part2 = true;
    let lines = read_file();
    
    parse_lines(lines, part2);

}

fn read_file() -> Vec<Vec<char>>{
    const INPUT_PATH: &str = "input.txt";

    let file = File::open(INPUT_PATH).expect("File invalid");
    let mut lines = BufReader::new(file).lines();
   
    let mut char_lines: Vec<Vec<char>> = Vec::new();
    let empty = Vec::<char>::new(); //add empty lines top and bottom
    char_lines.push(empty.clone());
    while let Some(line) = lines.next() {
        let single_line: Vec<char> = line.expect("string").chars().collect();
        char_lines.push(single_line);
    }
    char_lines.push(empty);
    return char_lines
}
