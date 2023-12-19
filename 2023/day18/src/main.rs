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
    // part_one();
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


    if let Ok(lines) = read_lines(r"E:\rusty\adventofcode\2023\day18\input.txt") {
        for line in lines {
            if let Ok(l) = line {
                let re = Regex::new(r"^[LDUR] \d+ \(#(\w{5})(\w)\)$").unwrap();
                let caps = re.captures(&l).unwrap();
                
                let stepstr = caps[1].to_string();
                let nsteps = i128::from_str_radix(&stepstr, 16).unwrap();
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
                dirs.push(dir);
                steps.push(nsteps);
             }
        }
    }


    let mut stepmap: HashMap<i128, Vec<i128>> = HashMap::new();   // row to vec<col>
    let mut pos: (i128, i128) = (100_000_000_000, 100_000_000_000);
    stepmap.entry(pos.0).or_insert(Vec::new()).push(pos.1);

    for i in 0..dirs.len() {
        println!("row {} of {} in filling", i, dirs.len());
        let moves = generate_moves(dirs[i], steps[i]);

        for m in moves {
            pos.0 += m.0;
            pos.1 += m.1;
            stepmap.entry(pos.0).or_insert(Vec::new()).push(pos.1);
        }
    }

    let smlen = stepmap.len();
    let mut counter: i128 = 0;
    for (_, values) in stepmap.iter_mut() {
        println!("sorting {} of {}", counter, smlen);
        counter += 1;

        values.sort();
    }


    let mut sum: i128 = 0;
    counter = 0;
    for (_, vect) in stepmap {
        println!("sorting {} of {}", counter, smlen);
        counter += 1;
        for i in (0..vect.len()).step_by(2) {
            if i + 1 >= vect.len() - 1 {
                sum += 1;
            } else {
                sum += vect[i+1] - vect[i] + 1;
            }
        }
    }


    println!("part 2 {sum}");
    // 82347690 too low 
    
}


// output is wrapped in a Result
// returns an Iterator to he Reader of lines of the file
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}


