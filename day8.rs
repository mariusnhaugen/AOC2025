use aoc::{read_example_as_lines}; // read_input_as_lines};

#[derive(PartialEq, Clone, Copy)]
struct BoxPosition(u64,u64,u64);


#[derive(PartialEq, Clone, Copy)]
struct BoxConnection{
    dist: f32,
    box1: BoxPosition,
    box2: BoxPosition,
}

fn solve(input: Vec<String>) -> usize {
    let connection_amount = 10;
    let mut connections_left  = connection_amount;
    let mut circuits = Vec::<Vec<BoxPosition>>::new();
    let mut all_boxes = Vec::<BoxPosition>::new();

    for line in input {
        let pos_vec: Vec<u64> = line.split(",").map(|x| x.parse::<u64>().unwrap()).collect();
        let [a,b,c] = pos_vec.try_into().expect("Three items");
        let boxpos = BoxPosition(a, b, c);
        all_boxes.push(boxpos);
    }

    let mut distances: Vec<BoxConnection> = Vec::new();
    for current_box in &all_boxes {
        //ISSUE: needs to consider all lengths to other boxes, not strictly shortest.
        let (dist,closest_box) = closest(&all_boxes, current_box);
        distances.push( BoxConnection { dist: dist, box1: *current_box,box2: closest_box },)
    }

    distances.sort_by(|a,b| (a.dist).total_cmp(&b.dist));
    distances.dedup_by(|a,b| a.dist == b.dist);

    for entry in &distances {
        println!("{} {} {} - {} {} {}  - Dist: {}", entry.box1.0,entry.box1.1,entry.box1.2 , entry.box2.0,entry.box2.1,entry.box2.2, entry.dist);
    }
    println!("");
    let wtf = BoxPosition{0:425, 1:690,2:689 };
    let wtf2 = BoxPosition{0:431,1:825,2:988 };
    println!("cdist {}", distance(&wtf, &wtf2));

    while connections_left > 0 {
        let connection = distances.get(connection_amount-connections_left).unwrap();
        let mut create_new_circuit = true;
        
        for circuit in circuits.iter_mut() {
            let found = circuit.iter().any(|junction_box| {
                *junction_box == connection.box1 || *junction_box == connection.box2
            });

            if found {
                println!("{} {} {} added to old circuit", connection.box1.0,connection.box1.1,connection.box1.2);
                circuit.push(connection.box1);
                create_new_circuit = false;
                break; 
            }

        }
        if create_new_circuit {
            println!("New : {} {} {} - {} {} {}", connection.box1.0,connection.box1.1,connection.box1.2 , connection.box2.0,connection.box2.1,connection.box2.2);
            circuits.push(vec![connection.box1, connection.box2]);
        }
        connections_left -= 1;
    }




    let mut circuit_lengths = Vec::new();
    for circuit in circuits {
        println!("Circuit: {}", circuit.len());
        circuit_lengths.push(circuit.len());
    }

    circuit_lengths.sort_by(|a,b| b.cmp(a));
    circuit_lengths.iter().take(3).product()

    
}


// fn solve_2(input: Vec<Vec<String>>) -> u64 {
// }

fn closest (boxes: &Vec<BoxPosition>, target: &BoxPosition) -> (f32, BoxPosition) {
    let mut closest = BoxPosition(0,0,0);
    let mut lowest_dist = f32::MAX;

    for junction_box in boxes {
        if junction_box != target {
            let cur_dist = distance(target, &junction_box);
            if cur_dist < lowest_dist { 
                lowest_dist= cur_dist;
                closest = junction_box.clone();

            }; } } 
    (lowest_dist, closest)
}

fn distance(a: &BoxPosition, b: &BoxPosition) -> f32 {
    let i = a.0.abs_diff(b.0).pow(2);
    let j = a.1.abs_diff(b.1).pow(2);
    let k = a.2.abs_diff(b.2).pow(2);
    let x = (i+j+k) as f64;
    x.sqrt() as f32
}

fn main (){
    // let input = read_input_as_lines();
    let input = read_example_as_lines();
    // let input_2 = input.clone();

    let part1 = solve(input);
    println!("Part 1: {}", part1);

    // let part2 = solve_2(input_2);
    // println!("Part 2: {}", part2);
}
