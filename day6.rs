use aoc::read_input_as_lines;

fn split_string(lines: &Vec<String>) -> Vec<Vec<String>> {
    let mut chars: Vec<Vec<String>> = Vec::new();
    
    for l in lines {
        let x = l.split_whitespace().map(String::from).collect();
        chars.push(x);
    }
    chars
}

fn strings_to_char(lines: Vec<String>) -> Vec<Vec<String>> {
    let mut chars: Vec<Vec<String>> = Vec::new();
    for l in lines {
        let x = l.chars().map(String::from).collect();
        chars.push(x);
    }
    chars
}

fn transpose2d(chars: Vec<Vec<String>>) -> Vec<Vec<String>>{
    let mut transposed = Vec::new();
    let columns = chars[0].len();
    for i in 0..columns {
        let mut p = Vec::new();
        for row in &chars {
            p.push(row[i].clone());
        } 
        transposed.push(p);
    }
    transposed
}

fn parse_cephalopod_math(lines: &mut Vec<Vec<String>>) -> Vec<Vec<String>>  {
    let mut translated_problems = Vec::new();
    
    let mut touched = 0;

    let o = lines.split_off(4);
    let operations = o[0].clone();
    let mut chunk_start = operations.len();
    let mut chunk_end; 


    for i in (0..operations.len()).rev() {
        if operations[i].contains("+") || operations[i].contains("*"){ 
            touched += 1;
            chunk_end = i;
            let rng = chunk_end..chunk_start;
            let v1 = &lines[0][rng.clone()];
            let v2 = &lines[1][rng.clone()];
            let v3 = &lines[2][rng.clone()];
            let v4 = &lines[3][rng.clone()];
            
            let mut problem_vec = Vec::new();
            problem_vec.push(v1.to_vec());
            problem_vec.push(v2.to_vec());
            problem_vec.push(v3.to_vec());
            problem_vec.push(v4.to_vec());

            let transposed = transpose2d(problem_vec);
            let mut problem = Vec::new();
            for line in transposed {
                problem.push(line.join("").trim().to_string());
            }
            problem.push(operations[i].clone());
            
            translated_problems.push(problem);
            chunk_start = if i>0 {i-1} else {0}; 
        }
    }

    println!("translate {} chunks.", touched);
    translated_problems 
}

fn solve_problems(problems: Vec<Vec<String>>) -> u64 {
    let mut sum = 0;  
    let mut touched = 0;
    let mut summed= 0;
    let mut prods= 0;

    for p in problems {
        let operator = &p[p.len()-1];
         
        let nums: Vec<u64> = p[0..p.len()-1].iter().map(|n| n.parse::<u64>().unwrap()).collect();
        touched += nums.len();

        if operator == "*" {
            prods += 1;
            sum += nums.iter().product::<u64>();
        } else if operator == "+" {
            summed += 1;
            sum += nums.iter().sum::<u64>();
        }

    }
    println!("touched {} numbers. {} sums and {} products", touched, summed, prods);
    sum
}

fn main (){
    let input = read_input_as_lines();
    let chars = split_string(&input);
    let problems = transpose2d(chars);
    let part1 = solve_problems(problems); //A problem looks like: [1,1,1,1,+]

    println!("Part 1: {}", part1);

    let mut chars2 = strings_to_char(input);
    let problems = parse_cephalopod_math(&mut chars2);
    let part2 = solve_problems(problems); 

    println!("Part 2: {}", part2);
}
