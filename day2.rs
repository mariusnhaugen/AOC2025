use std::fs::File;
use std::io::read_to_string;
//1 digit - ignore
//2 digits - chunks of 1
//3 digits - chunks of 1
//4 digits - chunks of 2
//5 digits - chunks of 1
//6 digits - chunks of 2,3
//7 digits - chunks of 1
//8 digits - chunks of 2,4
//9 digits - chunks of 3
//10 digits - chunks of 2,5 (checking 5v5 will find 1v1v1)

//n % 2  == 0 -- 2 and n/2
//n % 2  == 1 -- if n % 3 == 0 { triples } or singles 
//every odd number up to 20 is prime except 9 and 15, technically gets a bit fizzbuzzy for high
//digit counts but dataset seems to max at 10 digits

struct Range {
    start: String,
    end: String
}
fn get_file() -> String {
    const INPUT_PATH: &str = "input.txt";

    let f = File::open(INPUT_PATH).unwrap();
    let content = read_to_string(f);
    content.expect("valid file")
}

fn main() {
    let input = get_file();
    
    let rangestrings:Vec<&str> = input.trim().split(",").collect();
    
    let mut ranges = Vec::<Range>::new();

    for rangestring in rangestrings {
        let temp:Vec<&str> = rangestring.split("-").collect();
        ranges.push(Range {start:temp[0].to_string(), end:temp[1].to_string()});
        
    }
    

    let mut running_sum= 0;
    for x in ranges{
        let start = x.start.clone();
        let end = x.end.clone();
        let change = parse_range(x);
        println!("Parsed {} - {:<10} - {:>10} - {:>10}", start, end, change, running_sum );
        println!("  ");
        running_sum += change;
    }

    println!("End sum: {}", running_sum)
}

fn parse_range(range: Range) -> i64 {
    let start: i64 = range.start.parse().expect("expected number string"); 
    let end: i64 = range.end.parse().expect("expected number string"); 
    let mut sum = 0;


    for n in start..=end {
        let numstr = n.to_string();

        //special handling for 1 and 2 long digits
        if numstr.len() == 1 { continue; }
        if numstr.len() == 2 { 
            if compare_single(numstr){ 
                println!("o2 - {}", n);
                sum += n; 
            } 
            continue;
        }

        if numstr.len() % 2 == 0 {
            //even number of digits
            if compare_half(numstr.clone()) {
                println!("1/2 - {}", n);
                sum += n;
                continue;
            }
            if compare_two(numstr) {
                println!("2 - {}", n);
                sum += n;
            }
        }
        else {
            //odd number of digits
            if numstr.len() > 3 && numstr.len()  % 3 == 0 {
                if compare_three(numstr) {
                    println!("3 - {}", n);
                    sum += n;
                }
            } else {
                if compare_single(numstr) {
                    println!("1 - {}", n);
                    sum += n;
                }
            }
        }
    }
    sum
}

fn compare_single(strng: String) -> bool {
        let first_chunk = strng[..1].as_bytes();
        strng.as_bytes().chunks(1).all(|c| c == first_chunk)
}

fn compare_two(strng: String) -> bool {
    let first_chunk = strng[..2].as_bytes();
    strng.as_bytes().chunks(2).all(|c| c == first_chunk)
}

fn compare_three(strng: String) -> bool {
    let first_chunk = strng[..3].as_bytes();
    strng.as_bytes().chunks(3).all(|c| c == first_chunk)
}


fn compare_half(strng: String) -> bool {
        let (a,b) = strng.split_at(strng.len()/2);
        a.eq(b)
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_single() {
        assert!(compare_single("2222".to_string()));
        assert!(compare_single("2222222222".to_string()));
    }
    
    #[test]
    fn invalid_single() {
        assert!(!compare_single("2".to_string()));
        assert!(!compare_single("22232222".to_string()));
    }

    #[test]
    fn valid_pairs() {
        assert!(compare_two("2222".to_string()));
        assert!(compare_two("22222222".to_string()));
        assert!(compare_two("2222222222".to_string()));
    }

    #[test]
    fn valid_triples() {
        assert!(compare_three("232232".to_string()));
        assert!(compare_three("123123123".to_string()));
        assert!(compare_three("222222222".to_string()));
    }
    
    #[test]
    fn invalid_pairs() {
        assert!(!compare_two("2232".to_string()));
        assert!(!compare_two("22332222".to_string()));
        assert!(!compare_two("2222233333".to_string()));
    }
    
    #[test]
    fn valid_half() {
        assert!(compare_half("123123".to_string()));
        assert!(compare_half("22232223".to_string()));
        assert!(compare_half("1234512345".to_string()));
    }
    
    #[test]
    fn invalid_half() {
        assert!(!compare_half("124123".to_string()));
        assert!(!compare_half("22252223".to_string()));
        assert!(!compare_half("1233512345".to_string()));
    }
}


