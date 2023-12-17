use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::time::{Duration, Instant};
use itertools::Itertools;


fn main() {
    println!("Hello, world!");
    let now = Instant::now();
    println!("Hello, world!");
    part_one();
    println!("part 1 elapsed time: {}", now.elapsed().as_micros() as f64 / 100_000f64);

    let now = Instant::now();
    part_two(); 
    println!("part 2 elapsed time: {}", now.elapsed().as_micros() as f64 / 100_000f64);
}


fn part_one() {
    let mut map: Vec<Vec<bool>> = Vec::new();
    if let Ok(lines) = read_lines(r"E:\rusty\adventofcode\2023\day11\input.txt") {
        for line in lines {
            if let Ok(l) = line {
                let mut localvec: Vec<bool> = Vec::new();
                for c in l.trim().chars() {
                    if c == '#' {
                        localvec.push(true);
                    } else {
                        localvec.push(false);
                    }
                }
                map.push(localvec);

            }
        }
    }
    
    // expand map
    assert!(map.len() > 0);
    assert!(map[0].len() > 0);

    let h = map.len();
    let w: usize = map[0].len();

    let mut emptyrows: Vec<usize> = Vec::new();
    let mut emptycols: Vec<usize> = Vec::new();

    for r in 0..h {
        if map[r].clone().iter().any(|x| x == &true) {
            continue;
        }
        emptyrows.push(r);
    }

    for c in 0..w {
        let mut empty = true;
        for r in 0..h {
             if map[r][c] {
                empty = false;
                break;
             }
        }
        if empty {
            emptycols.push(c);
        }
    }

    for r in emptyrows.iter().rev() {
        map.insert(*r, map[*r].clone());
    }

    for c in emptycols.iter().rev() {
        for row in &mut map {
            row.insert(*c, row[*c])
        }
    }

    println!("\n\n");
    // for row in &map {
    //     println!("{:?}", row);
    // }
    // println!("\n\n");

    // find galaxies
    let mut points: Vec<(i128, i128)> = Vec::new();   // (row, col)  row:top->bottom, col:left->right

    for (r, row) in map.iter().enumerate() {
        for (c, elem) in row.iter().enumerate() {
            if *elem {
                points.push((r as i128, c as i128));
            }
        }
    }

    let mut sum: i128 = 0;
    for pair in points.iter().combinations(2) {
        let a = pair[0];
        let b = pair[1];

        sum += i128::abs(a.0 - b.0) + i128::abs(a.1 - b.1);

    }


    println!("part 1: {sum}");


}



fn part_two() {
    let mut map: Vec<Vec<bool>> = Vec::new();
    if let Ok(lines) = read_lines(r"E:\rusty\adventofcode\2023\day11\input.txt") {
        for line in lines {
            if let Ok(l) = line {
                let mut localvec: Vec<bool> = Vec::new();
                for c in l.trim().chars() {
                    if c == '#' {
                        localvec.push(true);
                    } else {
                        localvec.push(false);
                    }
                }
                map.push(localvec);

            }
        }
    }
    
    // get empty rows/cols
    assert!(map.len() > 0);
    assert!(map[0].len() > 0);

    let h = map.len();
    let w: usize = map[0].len();

    let mut emptyrows: Vec<usize> = Vec::new();
    let mut emptycols: Vec<usize> = Vec::new();

    for r in 0..h {
        if map[r].clone().iter().any(|x| x == &true) {
            continue;
        }
        emptyrows.push(r);
    }

    for c in 0..w {
        let mut empty = true;
        for r in 0..h {
             if map[r][c] {
                empty = false;
                break;
             }
        }
        if empty {
            emptycols.push(c);
        }
    }

    // find galaxies
    let mut points: Vec<(i128, i128)> = Vec::new();   // (row, col)  row:top->bottom, col:left->right

    for (r, row) in map.iter().enumerate() {
        for (c, elem) in row.iter().enumerate() {
            if *elem {
                points.push((r as i128, c as i128));
            }
        }
    }

    let mut sum: i128 = 0;
    for pair in points.iter().combinations(2) {
        let a = pair[0];
        let b = pair[1];

        sum += i128::abs(a.0 - b.0) + i128::abs(a.1 - b.1);
        // add the 1million-1 extra for empty rows/cols
        let nrowsinside = emptyrows.iter()
                                          .filter(|&x| 
                                                            (*x as i128) > i128::min(a.0, b.0) 
                                                         && (*x as i128) < i128::max(a.0, b.0) )
                                          .count();
        let ncolsinside = emptycols.iter()
                                          .filter(|&x| 
                                                            (*x as i128) > i128::min(a.1, b.1) 
                                                         && (*x as i128) < i128::max(a.1, b.1) )
                                          .count();
        sum += 999_999 * (nrowsinside as i128 + ncolsinside as i128);

    }


    println!("part 2: {sum}");

}






// output is wrapped in a Result
// returns an Iterator to he Reader of lines of the file
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}







// --- Day 11: Cosmic Expansion ---
// You continue following signs for "Hot Springs" and eventually come across an observatory. The Elf within turns out to be a researcher studying cosmic expansion using the giant telescope here.

// He doesn't know anything about the missing machine parts; he's only visiting for this research project. However, he confirms that the hot springs are the next-closest area likely to have people; he'll even take you straight there once he's done with today's observation analysis.

// Maybe you can help him with the analysis to speed things up?

// The researcher has collected a bunch of data and compiled the data into a single giant image (your puzzle input). The image includes empty space (.) and galaxies (#). For example:

// ...#......
// .......#..
// #.........
// ..........
// ......#...
// .#........
// .........#
// ..........
// .......#..
// #...#.....
// The researcher is trying to figure out the sum of the lengths of the shortest path between every pair of galaxies. However, there's a catch: the universe expanded in the time it took the light from those galaxies to reach the observatory.

// Due to something involving gravitational effects, only some space expands. In fact, the result is that any rows or columns that contain no galaxies should all actually be twice as big.

// In the above example, three columns and two rows contain no galaxies:

//    v  v  v
//  ...#......
//  .......#..
//  #.........
// >..........<
//  ......#...
//  .#........
//  .........#
// >..........<
//  .......#..
//  #...#.....
//    ^  ^  ^
// These rows and columns need to be twice as big; the result of cosmic expansion therefore looks like this:

// ....#........
// .........#...
// #............
// .............
// .............
// ........#....
// .#...........
// ............#
// .............
// .............
// .........#...
// #....#.......
// Equipped with this expanded universe, the shortest path between every pair of galaxies can be found. It can help to assign every galaxy a unique number:

// ....1........
// .........2...
// 3............
// .............
// .............
// ........4....
// .5...........
// ............6
// .............
// .............
// .........7...
// 8....9.......
// In these 9 galaxies, there are 36 pairs. Only count each pair once; order within the pair doesn't matter. For each pair, find any shortest path between the two galaxies using only steps that move up, down, left, or right exactly one . or # at a time. (The shortest path between two galaxies is allowed to pass through another galaxy.)

// For example, here is one of the shortest paths between galaxies 5 and 9:

// ....1........
// .........2...
// 3............
// .............
// .............
// ........4....
// .5...........
// .##.........6
// ..##.........
// ...##........
// ....##...7...
// 8....9.......
// This path has length 9 because it takes a minimum of nine steps to get from galaxy 5 to galaxy 9 (the eight locations marked # plus the step onto galaxy 9 itself). Here are some other example shortest path lengths:

// Between galaxy 1 and galaxy 7: 15
// Between galaxy 3 and galaxy 6: 17
// Between galaxy 8 and galaxy 9: 5
// In this example, after expanding the universe, the sum of the shortest path between all 36 pairs of galaxies is 374.

// Expand the universe, then find the length of the shortest path between every pair of galaxies. What is the sum of these lengths?

// Your puzzle answer was 10313550.

// --- Part Two ---
// The galaxies are much older (and thus much farther apart) than the researcher initially estimated.

// Now, instead of the expansion you did before, make each empty row or column one million times larger. That is, each empty row should be replaced with 1000000 empty rows, and each empty column should be replaced with 1000000 empty columns.

// (In the example above, if each empty row or column were merely 10 times larger, the sum of the shortest paths between every pair of galaxies would be 1030. If each empty row or column were merely 100 times larger, the sum of the shortest paths between every pair of galaxies would be 8410. However, your universe will need to expand far beyond these values.)

// Starting with the same initial image, expand the universe according to these new rules, then find the length of the shortest path between every pair of galaxies. What is the sum of these lengths?

// Your puzzle answer was 611998089572.

// Both parts of this puzzle are complete! They provide two gold stars: **