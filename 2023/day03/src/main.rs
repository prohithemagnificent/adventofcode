
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::cmp;
// use std::collections::HashMap;
use multimap::MultiMap;

fn main() {
    println!("Hello, world!");
    part_one();
    part_two();

}


fn part_one() {
    let mut _sum: u128= 0;

    let mut allparts: Vec<Vec<char>> = vec![];
    // if let Ok(lines) = read_lines(r".\input.txt") {
    if let Ok(lines) = read_lines(r"E:\rusty\adventofcode\2023\day03\input.txt") {
       
        for line in lines {
            if let Ok(l) = line {
                let linvec: Vec<char> = l.chars().collect();
                allparts.push(linvec);
            }
        }
    }

    let height: usize = allparts.len();
    let width: usize = allparts[0].len();

    let digits: Vec<char> = vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

    for (nrow, row) in allparts.iter().enumerate() {
        let mut ncol = 0;
        while ncol <= width-1 {
            let c: char = row[ncol];
            if digits.contains(&c) {
                let fromcol = ncol;
                let mut tocol: usize = ncol;
                for i in fromcol..width {
                    if digits.contains(&row[i]) {
                        tocol = i;
                    } else {
                        break;
                    }
                } 
                _sum += check_surroundings(&allparts, nrow, fromcol, tocol, height, width);
                ncol = tocol;

            }
            ncol += 1;
        }
    }
    println!("part 1: {_sum}")

}




fn check_surroundings(allparts: &Vec<Vec<char>>, 
                      nrow: usize, 
                      fromcol: usize, 
                      tocol: usize,
                      height: usize,
                      width: usize) 
                      -> u128 {
    // height, width = number of rows/columns, max index is 1 less
    // returns 0 if there is no symbol around the number
    // returns the number if there is
    // println!("{nrow} {fromcol} {tocol}");
    let nonsymbols: Vec<char> = vec!['.', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
    let mut symbolexists = false;

    // unsigned usize vs 0-1 hack :(
    let mut fromcol_local = fromcol;
    if fromcol == 0 {
        fromcol_local = 1
    }
    // row above
    if nrow > 0 {
        for col in cmp::max(0, fromcol_local - 1)..=cmp::min(width - 1, tocol + 1) {
            if !(nonsymbols.contains(&allparts[nrow - 1][col])) {
                symbolexists = true;
            }
        }
    }
    // row below
    if nrow < height - 1 {
        for col in cmp::max(0, fromcol_local - 1)..=cmp::min(width - 1, tocol + 1) {
            if !(nonsymbols.contains(&allparts[nrow + 1][col])) {
                symbolexists = true;
            }
        }
    }
    // same row
    if fromcol > 0 {
        if !(nonsymbols.contains(&allparts[nrow][fromcol - 1])) {
            symbolexists = true;
        }
    }
    if tocol < width - 1 {
        if !(nonsymbols.contains(&allparts[nrow][tocol + 1])) {
            symbolexists = true;
        }
    }

    if symbolexists {
        let mut retval: u128 = 0;
        for i in 0..=tocol - fromcol {
            let digit: u32 = allparts[nrow][tocol - i].to_digit(10).unwrap();
            let exp: u32 = i as u32; // i like to live dangerously
            retval += 10u128.pow(exp) * u128::from(digit);
        }
        return retval;
    } else {
        return 0u128;
    }

}



fn part_two() {
    let mut _sumofproducts: u128= 0;
    let mut gear_ratio_parts: MultiMap<(usize,usize), u128> = MultiMap::new();

    let mut allparts: Vec<Vec<char>> = vec![];
    // if let Ok(lines) = read_lines(r".\input.txt") {
    if let Ok(lines) = read_lines(r"E:\rusty\adventofcode\2023\day03\input.txt") {
       
        for line in lines {
            if let Ok(l) = line {
                let linvec: Vec<char> = l.chars().collect();
                allparts.push(linvec);
            }
        }
    }

    let height: usize = allparts.len();
    let width: usize = allparts[0].len();

    let digits: Vec<char> = vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

    for (nrow, row) in allparts.iter().enumerate() {
        let mut ncol = 0;
        while ncol <= width-1 {
            let c: char = row[ncol];
            if digits.contains(&c) {
                let fromcol = ncol;
                let mut tocol: usize = ncol;
                for i in fromcol..width {
                    if digits.contains(&row[i]) {
                        tocol = i;
                    } else {
                        break;
                    }
                } 
                let (pos, val) = check_surroundings_for_gear(&allparts, nrow, fromcol, tocol, height, width);
                gear_ratio_parts.insert(pos, val);
                ncol = tocol;

            }
            ncol += 1;
        }
    }

    for (_, vals) in gear_ratio_parts {
        if vals.len() == 2 {
            _sumofproducts += vals[0] * vals[1];
        }
    }
    println!("part 2: {_sumofproducts}")

}




fn check_surroundings_for_gear(allparts: &Vec<Vec<char>>, 
                      nrow: usize, 
                      fromcol: usize, 
                      tocol: usize,
                      height: usize,
                      width: usize) 
                      -> ((usize,usize), u128) {
    // height, width = number of rows/columns, max index is 1 less
    // returns 0 if there is no symbol around the number
    // returns the number if there is
    // println!("{nrow} {fromcol} {tocol}");
    let mut symbolexists = false;
    let mut pos_of_mult_sign = (0usize, 0usize);

    // unsigned usize vs 0-1 hack :(
    let mut fromcol_local = fromcol;
    if fromcol == 0 {
        fromcol_local = 1
    }
    // row above
    if nrow > 0 {
        for col in cmp::max(0, fromcol_local - 1)..=cmp::min(width - 1, tocol + 1) {
            if allparts[nrow - 1][col] == '*' {
                symbolexists = true;
                pos_of_mult_sign = (nrow - 1, col);
            }
        }
    }
    // row below
    if nrow < height - 1 {
        for col in cmp::max(0, fromcol_local - 1)..=cmp::min(width - 1, tocol + 1) {
            if allparts[nrow + 1][col] == '*' {
                symbolexists = true;
                pos_of_mult_sign = (nrow + 1, col);
            }
        }
    }
    // same row
    if fromcol > 0 {
        if allparts[nrow][fromcol - 1] == '*' {
            symbolexists = true;
            pos_of_mult_sign = (nrow, fromcol - 1);

        }
    }
    if tocol < width - 1 {
        if allparts[nrow][tocol + 1] == '*' {
            symbolexists = true;
            pos_of_mult_sign = (nrow, tocol + 1);
        }
    }

    if symbolexists {
        let mut retval: u128 = 0;
        for i in 0..=tocol - fromcol {
            let digit: u32 = allparts[nrow][tocol - i].to_digit(10).unwrap();
            let exp: u32 = i as u32; // i like to live dangerously
            retval += 10u128.pow(exp) * u128::from(digit);
        }
        return (pos_of_mult_sign, retval);
    } else {
        return ((0usize, 0usize), 0u128);
    }

}




// output is wrapped in a Result
// returns an Iterator to he Reader of lines of the file
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}




// --- Day 3: Gear Ratios ---
// You and the Elf eventually reach a gondola lift station; he says the gondola lift will take you up to the water source, but this is as far as he can bring you. You go inside.

// It doesn't take long to find the gondolas, but there seems to be a problem: they're not moving.

// "Aaah!"

// You turn around to see a slightly-greasy Elf with a wrench and a look of surprise. "Sorry, I wasn't expecting anyone! The gondola lift isn't working right now; it'll still be a while before I can fix it." You offer to help.

// The engineer explains that an engine part seems to be missing from the engine, but nobody can figure out which one. If you can add up all the part numbers in the engine schematic, it should be easy to work out which part is missing.

// The engine schematic (your puzzle input) consists of a visual representation of the engine. There are lots of numbers and symbols you don't really understand, but apparently any number adjacent to a symbol, even diagonally, is a "part number" and should be included in your sum. (Periods (.) do not count as a symbol.)

// Here is an example engine schematic:

// 467..114..
// ...*......
// ..35..633.
// ......#...
// 617*......
// .....+.58.
// ..592.....
// ......755.
// ...$.*....
// .664.598..
// In this schematic, two numbers are not part numbers because they are not adjacent to a symbol: 114 (top right) and 58 (middle right). Every other number is adjacent to a symbol and so is a part number; their sum is 4361.

// Of course, the actual engine schematic is much larger. What is the sum of all of the part numbers in the engine schematic?

// Your puzzle answer was 526404.

// --- Part Two ---
// The engineer finds the missing part and installs it in the engine! As the engine springs to life, you jump in the closest gondola, finally ready to ascend to the water source.

// You don't seem to be going very fast, though. Maybe something is still wrong? Fortunately, the gondola has a phone labeled "help", so you pick it up and the engineer answers.

// Before you can explain the situation, she suggests that you look out the window. There stands the engineer, holding a phone in one hand and waving with the other. You're going so slowly that you haven't even left the station. You exit the gondola.

// The missing part wasn't the only issue - one of the gears in the engine is wrong. A gear is any * symbol that is adjacent to exactly two part numbers. Its gear ratio is the result of multiplying those two numbers together.

// This time, you need to find the gear ratio of every gear and add them all up so that the engineer can figure out which gear needs to be replaced.

// Consider the same engine schematic again:

// 467..114..
// ...*......
// ..35..633.
// ......#...
// 617*......
// .....+.58.
// ..592.....
// ......755.
// ...$.*....
// .664.598..
// In this schematic, there are two gears. The first is in the top left; it has part numbers 467 and 35, so its gear ratio is 16345. The second gear is in the lower right; its gear ratio is 451490. (The * adjacent to 617 is not a gear because it is only adjacent to one part number.) Adding up all of the gear ratios produces 467835.

// What is the sum of all of the gear ratios in your engine schematic?

// Your puzzle answer was 84399773.

// Both parts of this puzzle are complete! They provide two gold stars: **