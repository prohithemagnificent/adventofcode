use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


fn main() {
    println!("Hello, world!");

    part_one();
    part_two();

    // 54585   your answer is too low
    // 54749 your answer is too high
}



fn part_one() {
    let mut _sum: u128= 0;
    if let Ok(lines) = read_lines(r".\input.txt") {
        for line in lines {
            let mut first = 0;
            let mut last = 0;
            if let Ok(l) =  line {
                let lvec: Vec<char> = l.chars().collect();
                for c in lvec.iter() {
                    if c.is_digit(10) {
                        if let Some(digit) = c.to_digit(10) {
                            first = digit;
                            break;
                        }
                    }
                }
                for c in lvec.iter().rev() {
                    if c.is_digit(10) {
                        if let Some(digit) = c.to_digit(10) {
                            last = digit;
                            break;
                        }
                    }
                }
                _sum += 10 * u128::from(first) + u128::from(last);
            }
        }
    }
    println!("the sum is {_sum}");
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct DigitPos {
    pos: usize,
    digit: u32
}


fn part_two() {
    let mut _sum: u128= 0;
    if let Ok(lines) = read_lines(r".\input.txt") {
        println!("\n\n");
        for line in lines {
            let mut firsts: Vec<DigitPos> = vec![];
            let mut lasts: Vec<DigitPos> = vec![];
            if let Ok(l) =  line {
                // println!("{l}");
                // find digits
                let lvec: Vec<char> = l.chars().collect();
                for (pos, c) in lvec.iter().enumerate() {
                    if c.is_digit(10) {
                        if let Some(digit) = c.to_digit(10) {
                            firsts.push(DigitPos { pos: pos, digit: digit });
                            break;
                        }
                    }
                }
                let lvec: Vec<char> = l.chars().collect();
                for (pos, c) in lvec.iter().rev().enumerate() {
                    if c.is_digit(10) {
                        if let Some(digit) = c.to_digit(10) {
                            lasts.push(DigitPos { pos: lvec.len() - pos - 1, digit: digit });
                            break;
                        }
                    }
                }

                // find words
                for (num_str, num) in [("one", 1), ("two", 2), 
                                                  ("three", 3), ("four", 4), 
                                                  ("five", 5), ("six", 6), 
                                                  ("seven", 7), ("eight", 8),
                                                  ("nine", 9), ("zero", 0)] {
                    if l.contains(num_str) {
                        firsts.push(find_first_num_in_str(&l, num_str, num));
                        lasts.push(find_last_num_in_str(&l, num_str, num))
                    }
                }
                firsts.sort();
                lasts.sort();
                lasts.reverse();
                let first: u32 = firsts[0].digit;
                let last: u32 = lasts[0].digit;
                // println!("    first={first}, last={last}");
                _sum += 10 * u128::from(first) + u128::from(last);
            }
        }
    }
    println!("PART2: the sum is {_sum}");
}




// output is wrapped in a Result
// returns an Iterator to he Reader of lines of the file
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}


fn find_first_num_in_str(line: &str, num_str: &str, num: u32) -> DigitPos {
    let v: Vec<_> = line.match_indices(num_str).collect();
    return DigitPos { pos: v[0].0, digit: num };
}


fn find_last_num_in_str(line: &str, num_str: &str, num: u32) -> DigitPos {
    let v: Vec<_> = line.rmatch_indices(&num_str).collect();
    // pos technically starts at the end of the word, but the word takse up all positions
    // so it doesn't matter, others will fall outside
    return DigitPos { pos: v[0].0, digit: num };
}