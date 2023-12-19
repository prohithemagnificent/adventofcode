use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::time::{Duration, Instant};
use std::usize;
use regex::Regex;


fn main() {
    println!("Hello, world!");
    let now = Instant::now();
    part_one();
    println!("part 1 elapsed time: {}", now.elapsed().as_micros() as f64 / 100_000f64);

    let now = Instant::now();
    part_two(); 
    println!("part 2 elapsed time: {}", now.elapsed().as_micros() as f64 / 100_000f64);
}




fn generate_moves(dir: char, steps: i128) -> Vec<(i128, i128)> {
    let mut ret: Vec<(i128, i128)> = vec![];
    if dir == 'U' {
        for _ in 1..=steps {
            ret.push((-1, 0));
        }
    }
    if dir == 'D' {
        for _ in 1..=steps {
            ret.push((1, 0));
        }
    }
    if dir == 'R' {
        for _ in 1..=steps {
            ret.push((0, 1));
        }
    }
    if dir == 'L' {
        for _ in 1..=steps {
            ret.push((0, -1));
        }
    }
    return ret;
}



fn fill_map_from(r_in: usize, c_in: usize, map: &mut Vec<Vec<bool>>, mapsize: usize) {

    let mut dq: VecDeque<(usize, usize)> = VecDeque::new();
    let mut bounds: HashSet<char> = HashSet::new();
    let mut this_region: Vec<(usize, usize)> = vec![];

    dq.push_back((r_in, c_in));

    while !dq.is_empty() {

        let (r, c) = dq.pop_front().expect("WTF");
        if map[r][c] {
            continue;
        }
        map[r][c] = true; // current region
        if r > 0 {
            dq.push_back((r-1, c));
        }
        if r < mapsize - 1 {
            dq.push_back((r+1, c));
        }
        if c > 0 {
            dq.push_back((r, c-1));
        }
        if c < mapsize - 1 {
            dq.push_back((r, c+1));
        }
    }
}



// ugly braindead solution
fn part_one() {
    // big enough map lol
    let mapsize: usize = 2000;
    let mut map: Vec<Vec<bool>> = vec![];

    let mut dirs: Vec<char> = vec![];
    let mut steps: Vec<i128> = vec![];
    let mut color: Vec<String> = vec![];

    for _ in 0..mapsize {
        map.push(vec![false; mapsize]);
    }
    if let Ok(lines) = read_lines(r"E:\rusty\adventofcode\2023\day18\input.txt") {
        for line in lines {
            if let Ok(l) = line {
                let re = Regex::new(r"^([LDUR]) (\d+) \(#(\w+)\)$").unwrap();
                let caps = re.captures(&l).unwrap();

                dirs.push(caps[1].to_string().chars().nth(0).unwrap());
                steps.push(caps[2].to_string().parse::<i128>().unwrap());
                color.push(caps[3].to_string());


             }
        }
    }

    let mut pos: (usize, usize) = (mapsize / 2, mapsize / 2);
    map[pos.0][pos.1] = true;

    for i in 0..dirs.len() {
        let moves = generate_moves(dirs[i], steps[i]);

        for m in moves {
            pos.0 = (pos.0 as i128 + m.0) as usize;
            pos.1 = (pos.1 as i128 + m.1) as usize;
            map[pos.0][pos.1] = true;
        }
    }

    let outline = map.clone();

    // fill outside
    fill_map_from(0, 0, &mut map, mapsize);

    // flip all bits
    for r in 0..mapsize {
        for c in 0..mapsize {
            map[r][c] = !map[r][c];
            // if map[r][c] {
            //     println!("{r} {c}");
            //     break;
            // }
        }
    }
    // add back outline
    for r in 0..mapsize {
        for c in 0..mapsize {
            if outline[r][c] {
                map[r][c] = true;
            }
        }
    }

    let mut filled: i128 = 0;
    for r in 0..mapsize {
        for c in 0..mapsize {
            if map[r][c] {
                filled += 1;
            }
        }
    }


    println!("part 1 {filled}");


}
fn part_two() {
    // big enough map lol

    let mut dirs: Vec<char> = vec![];
    let mut steps: Vec<i128> = vec![];

    let mut dist: i128 = 0;
    // if let Ok(lines) = read_lines(r"E:\rusty\adventofcode\2023\day18\test.txt") {
    if let Ok(lines) = read_lines(r"E:\rusty\adventofcode\2023\day18\input.txt") {
        for line in lines {
            if let Ok(l) = line {
                let re = Regex::new(r"^[LDUR] \d+ \(#(\w{5})(\w)\)$").unwrap();
                let caps = re.captures(&l).unwrap();
                
                let stepstr = caps[1].to_string();
                let nsteps = i128::from_str_radix(&stepstr, 16).unwrap();
                dist += nsteps;
                let mut dir = caps[2].to_string().chars().nth(0).unwrap();
                if dir == '0' {
                    dir = 'R'
                }
                if dir == '1' {
                    dir = 'D'
                }
                if dir == '2' {
                    dir = 'L'
                }
                if dir == '3' {
                    dir = 'U'
                }
                if dirs.len() > 0 && dirs.last().unwrap() == &dir {
                    *steps.last_mut().unwrap() += nsteps;
                } else {
                    dirs.push(dir);
                    steps.push(nsteps);
                }
            }
        }
    }


    // let mut pos: (i128, i128) = (100_000_000_000, 100_000_000_000);
    let mut pos: (i128, i128) = (0, 0);
    let initalpos = pos.clone();
    let mut corners: Vec<(i128, i128)> = Vec::new();
    corners.push(initalpos);

    

    for i in 0..dirs.len() {
        // println!("row {} of {} in filling", i, dirs.len());
        let moves = generate_moves(dirs[i], steps[i]);

        for m in moves {
            pos.0 += m.0;
            pos.1 += m.1;
        }
        corners.push(pos);

    }
    for c in &corners {
        // println!("{:?}", c);
    }
    // println!("len of corners {}", corners.len());

    let mut sum: i128 = 0;
    for i in 0..=corners.len() - 2 {
        sum += corners[i].1 * corners[i+1].0 - corners[i].0 * corners[i+1].1;
        // println!("multiplying {:?} and {:?}", corners[i], corners[i+1]);
    }
    sum += dist;
    sum /= 2;
    sum = i128::abs(sum);
    sum += 1;


    println!("part 2 {sum}");
    // 82347690 too low 
    // 6300192879494811220501840 too high
    // 85070681287976 too low
    // 85070763635665 incorrect
    
}


// output is wrapped in a Result
// returns an Iterator to he Reader of lines of the file
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}






// --- Day 18: Lavaduct Lagoon ---
// Thanks to your efforts, the machine parts factory is one of the first factories up and running since the lavafall came back. However, to catch up with the large backlog of parts requests, the factory will also need a large supply of lava for a while; the Elves have already started creating a large lagoon nearby for this purpose.

// However, they aren't sure the lagoon will be big enough; they've asked you to take a look at the dig plan (your puzzle input). For example:

// R 6 (#70c710)
// D 5 (#0dc571)
// L 2 (#5713f0)
// D 2 (#d2c081)
// R 2 (#59c680)
// D 2 (#411b91)
// L 5 (#8ceee2)
// U 2 (#caa173)
// L 1 (#1b58a2)
// U 2 (#caa171)
// R 2 (#7807d2)
// U 3 (#a77fa3)
// L 2 (#015232)
// U 2 (#7a21e3)
// The digger starts in a 1 meter cube hole in the ground. They then dig the specified number of meters up (U), down (D), left (L), or right (R), clearing full 1 meter cubes as they go. The directions are given as seen from above, so if "up" were north, then "right" would be east, and so on. Each trench is also listed with the color that the edge of the trench should be painted as an RGB hexadecimal color code.

// When viewed from above, the above example dig plan would result in the following loop of trench (#) having been dug out from otherwise ground-level terrain (.):

// #######
// #.....#
// ###...#
// ..#...#
// ..#...#
// ###.###
// #...#..
// ##..###
// .#....#
// .######
// At this point, the trench could contain 38 cubic meters of lava. However, this is just the edge of the lagoon; the next step is to dig out the interior so that it is one meter deep as well:

// #######
// #######
// #######
// ..#####
// ..#####
// #######
// #####..
// #######
// .######
// .######
// Now, the lagoon can contain a much more respectable 62 cubic meters of lava. While the interior is dug out, the edges are also painted according to the color codes in the dig plan.

// The Elves are concerned the lagoon won't be large enough; if they follow their dig plan, how many cubic meters of lava could it hold?

// Your puzzle answer was 35244.

// --- Part Two ---
// The Elves were right to be concerned; the planned lagoon would be much too small.

// After a few minutes, someone realizes what happened; someone swapped the color and instruction parameters when producing the dig plan. They don't have time to fix the bug; one of them asks if you can extract the correct instructions from the hexadecimal codes.

// Each hexadecimal code is six hexadecimal digits long. The first five hexadecimal digits encode the distance in meters as a five-digit hexadecimal number. The last hexadecimal digit encodes the direction to dig: 0 means R, 1 means D, 2 means L, and 3 means U.

// So, in the above example, the hexadecimal codes can be converted into the true instructions:

// #70c710 = R 461937
// #0dc571 = D 56407
// #5713f0 = R 356671
// #d2c081 = D 863240
// #59c680 = R 367720
// #411b91 = D 266681
// #8ceee2 = L 577262
// #caa173 = U 829975
// #1b58a2 = L 112010
// #caa171 = D 829975
// #7807d2 = L 491645
// #a77fa3 = U 686074
// #015232 = L 5411
// #7a21e3 = U 500254
// Digging out this loop and its interior produces a lagoon that can hold an impressive 952408144115 cubic meters of lava.

// Convert the hexadecimal color codes into the correct instructions; if the Elves follow this new dig plan, how many cubic meters of lava could the lagoon hold?

// Your puzzle answer was 85070763635666.

// Both parts of this puzzle are complete! They provide two gold stars: **