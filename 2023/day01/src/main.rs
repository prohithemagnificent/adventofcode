use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


fn main() {
    println!("Hello, world!");

    part_one();
    part_two();
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






// --- Day 1: Trebuchet?! ---
// Something is wrong with global snow production, and you've been selected to take a look. The Elves have even given you a map; on it, they've used stars to mark the top fifty locations that are likely to be having problems.

// You've been doing this long enough to know that to restore snow operations, you need to check all fifty stars by December 25th.

// Collect stars by solving puzzles. Two puzzles will be made available on each day in the Advent calendar; the second puzzle is unlocked when you complete the first. Each puzzle grants one star. Good luck!

// You try to ask why they can't just use a weather machine ("not powerful enough") and where they're even sending you ("the sky") and why your map looks mostly blank ("you sure ask a lot of questions") and hang on did you just say the sky ("of course, where do you think snow comes from") when you realize that the Elves are already loading you into a trebuchet ("please hold still, we need to strap you in").

// As they're making the final adjustments, they discover that their calibration document (your puzzle input) has been amended by a very young Elf who was apparently just excited to show off her art skills. Consequently, the Elves are having trouble reading the values on the document.

// The newly-improved calibration document consists of lines of text; each line originally contained a specific calibration value that the Elves now need to recover. On each line, the calibration value can be found by combining the first digit and the last digit (in that order) to form a single two-digit number.

// For example:

// 1abc2
// pqr3stu8vwx
// a1b2c3d4e5f
// treb7uchet
// In this example, the calibration values of these four lines are 12, 38, 15, and 77. Adding these together produces 142.

// Consider your entire calibration document. What is the sum of all of the calibration values?

// Your puzzle answer was 55971.

// --- Part Two ---
// Your calculation isn't quite right. It looks like some of the digits are actually spelled out with letters: one, two, three, four, five, six, seven, eight, and nine also count as valid "digits".

// Equipped with this new information, you now need to find the real first and last digit on each line. For example:

// two1nine
// eightwothree
// abcone2threexyz
// xtwone3four
// 4nineeightseven2
// zoneight234
// 7pqrstsixteen
// In this example, the calibration values are 29, 83, 13, 24, 42, 14, and 76. Adding these together produces 281.

// What is the sum of all of the calibration values?

// Your puzzle answer was 54719.

// Both parts of this puzzle are complete! They provide two gold stars: **