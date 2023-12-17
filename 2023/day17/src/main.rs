use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::time::{Duration, Instant};
use std::usize;
use priority_queue::PriorityQueue;


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


fn h(pos: &(usize, usize), height: &usize, width: &usize) -> i128 {
    return (*height as i128 - pos.0 as i128) + (*width as i128 - pos.1 as i128) ;
}

fn get_dir(curr: &(usize, usize), prev: &(usize, usize)) -> char {
    if curr.0 == prev.0 {
        if curr.1 > prev.1 {
            return 'R';
        } else {
            return 'L';
        }
    } else {
        if curr.0 > prev.0 {
            return 'D';
        } else {
            return 'U';
        }
    }
}




fn fourth_in_row(pos: &(usize, usize), prev: &(usize, usize), came_from: &HashMap<(usize, usize), (usize, usize)>) -> bool {
    if came_from.len() < 3 {
        return false;
    }
    let pos1 = prev;
    if let Some(pos2) = came_from.get(pos1) {
        if let Some(pos3) = came_from.get(pos2) {
            if let Some(pos4) = came_from.get(pos3) {
                let d = get_dir(pos, pos1);
                let d1 = get_dir(pos1, pos2);
                let d2 = get_dir(pos2, pos3);
                let d3 = get_dir(pos3, pos4);
                if d == d1 && d1 == d2 && d2 == d3  {
                    return true;
                }
            }
        }
    }
    return false;
}

fn neighbours(pos: &(usize, usize), height: &usize, width: &usize, came_from: &HashMap<(usize, usize), (usize, usize)> ) -> Vec<(usize, usize)> {
    let mut ret: Vec<(usize, usize)> = Vec::new();
    // R
    if pos.1 < width - 1 {
        let n = (pos.0 , pos.1 + 1);
        if !fourth_in_row(&n, &pos, came_from) {
            ret.push(n);
        }
    }
    // D
    if pos.0 < height - 1 {
        let n = (pos.0 + 1 , pos.1);
        if !fourth_in_row(&n, &pos, came_from) {
            ret.push(n);
        }
    }
    // U
    if pos.0 > 0 {
        let n = (pos.0 - 1 , pos.1);
        if !fourth_in_row(&n, &pos, came_from) {
            ret.push(n);
        }
    }
    // L
    if pos.1 > 0 {
        let n = (pos.0 , pos.1 - 1);
        if !fourth_in_row(&n, &pos, came_from) {
            ret.push(n);
        }
    }


    return ret;
}


fn reconstruct(from_pos: (usize, usize), came_from: &HashMap<(usize, usize), (usize, usize)>, map: &Vec<Vec<i128>>) -> i128 {
    let mut ret: i128 = 0;
    let mut pos = from_pos;

    let mut map2: Vec<Vec<i128>> = map.clone();
    while pos != (0,0) {
        map2[pos.0][pos.1] = 0;
        ret += map[pos.0][pos.1];
        pos = came_from[&pos];
    }
    for v in map2 {
        println!("{:?}", v);
    }
    return ret;
}


fn astar(map: &Vec<Vec<i128>>) -> i128 {
    let height = map.len();
    assert!(height > 0);
    let width = map[0].len();
    assert!(width > 0);


    // A*
    let start: (usize, usize) = (0, 0);
    let goal: (usize, usize) = (height - 1, width - 1);

    let mut open_set: PriorityQueue<(usize, usize), i128> = PriorityQueue::new();
    open_set.push(start, i128::MAX);
    let mut came_from: HashMap<(usize, usize), (usize, usize)> = HashMap::new();
    let mut gScore: HashMap<(usize, usize), i128> = HashMap::new();
    let mut fScore: HashMap<(usize, usize), i128> = HashMap::new();

    for r in 0..height {
        for c in 0..width {
            gScore.insert((r,c), i128::MAX);
            fScore.insert((r,c), i128::MAX);
        }
    }
    gScore.insert(start, 0);
    fScore.insert(start, h(&start, &height, &width));
 

    while !open_set.is_empty() {
        let (current, _) = open_set.pop().unwrap();
        if current == goal {
            return reconstruct(current, &came_from, map)
        }
        for n in neighbours(&current, &height, &width, &came_from) {
            // println!("current {} {}, neigh: {} {}", current.0, current.1, n.0, n.1);
            let tentative_gScore: i128 = gScore[&current] + map[n.0][n.1];
            if tentative_gScore < gScore[&n] {
                // println!("    updated with score {tentative_gScore}");
                came_from.insert(n, current);
                gScore.insert(n, tentative_gScore);
                fScore.insert(n, tentative_gScore + h(&n, &height, &width));
                open_set.push(n, i128::MAX - fScore[&n]); // priority is large is fScore is small!
            }
    
        }
    }
    return i128::MAX; // fail
}



fn part_one() {
    let mut map: Vec<Vec<i128>> = Vec::new();
    // if let Ok(lines) = read_lines(r"E:\rusty\adventofcode\2023\day17\input.txt") {
    if let Ok(lines) = read_lines(r"E:\rusty\adventofcode\2023\day17\test.txt") {
        for line in lines {
            if let Ok(l) = line {
                map.push(l.trim()
                          .chars()
                          .map(|c| c.to_digit(10u32).unwrap() as i128)
                          .collect());
            }
        }
    }

    map[0][0] = 0; // ??
    let res = astar(&map);
    assert!(res != i128::MAX);

    

    println!("part 1: {res}");
}



fn part_two() {

}


// output is wrapped in a Result
// returns an Iterator to he Reader of lines of the file
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}



// 1054 too low
// 1414 too high
// 1266 incorrect