use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::cmp;
use std::time::{Duration, Instant};


fn main() {
    let now = Instant::now();
    println!("Hello, world!");
    part_one();
    println!("part 1 elapsed time: {}", now.elapsed().as_micros() as f64 / 100_000f64);
    let now = Instant::now();
    // part_two(); // cant be bruteforced lol
    println!("part 2 elapsed time: {}", now.elapsed().as_micros() as f64 / 100_000f64);
}


struct MapElement {
    min: i128,
    max: i128 // inclusive
}

impl MapElement{
    fn contains(&self, num: i128) -> bool {
        return num >= self.min && num <= self.max;
    }
}

struct Map {
    from_elems: Vec<MapElement>,
    to_elems_offset: Vec<i128>
}

impl Map {
    fn add_elements(&mut self, element_from: MapElement, element_to_offset: i128) {
        self.from_elems.push(element_from);
        self.to_elems_offset.push(element_to_offset);
    }

    fn translate(&self, num: i128) -> i128 {
        for (n, elem) in self.from_elems.iter().enumerate() {
            if elem.contains(num) {
                return (num as i128 + self.to_elems_offset[n]) as i128;
            }
        }
        return num;
    }
}


fn part_one() {
    let mut min_loc: i128 = i128::MAX;
    let mut maps: Vec<Map> = vec![];
    let mut seeds: Vec<i128> = vec![];
    // if let Ok(lines) = read_lines(r".\input.txt") {
    if let Ok(lines) = read_lines(r"E:\rusty\adventofcode\2023\day05\input.txt") {
        
        for line in lines {
            if let Ok(l) = line {
                if l.contains("seeds:") {
                    let splitseedl: Vec<&str> = l.trim().split(' ').collect();
                    for i in 1..splitseedl.len() {
                        seeds.push(splitseedl[i].parse().unwrap());
                    }
                    continue;
                }
                if l == "" {
                    continue;
                }
                if l.contains("map:") {
                    maps.push(Map{from_elems: vec![], to_elems_offset: vec![]});
                    continue;
                }
                let splitl: Vec<&str> = l.trim().split(' ').collect();

                let to: i128 = splitl[0].parse().unwrap();
                let from: i128 = splitl[1].parse().unwrap();
                let length: i128 = splitl[2].parse().unwrap();

                let mlen = maps.len();
                maps[mlen - 1].add_elements(MapElement {min: from, max: from + length - 1}, to - from );
                

            }
        }
    }
    for seed in seeds {
        let mut seedn = seed;
        for map in &maps {
            seedn = map.translate(seedn);
        }
        min_loc = cmp::min(min_loc, seedn);
    }
    println!("part1: {min_loc}")
}


fn part_two() {
    // let mut min_loc: i128 = i128::MAX;
    // let mut maps: Vec<Map> = vec![];
    // let mut seeds: Vec<i128> = vec![];
    // // if let Ok(lines) = read_lines(r".\input.txt") {
    // if let Ok(lines) = read_lines(r"E:\rusty\adventofcode\2023\day05\input.txt") {
        
    //     for line in lines {
    //         if let Ok(l) = line {
    //             if l.contains("seeds:") {
    //                 let splitseedl: Vec<&str> = l.trim().split(' ').collect();
    //                 for i in 1..splitseedl.len() {
    //                     if i % 2 == 1 {
    //                         continue; // skip every 2nd
    //                     }
    //                     let seed_from: i128 = splitseedl[i].parse().unwrap();
    //                     let seed_range: i128 = splitseedl[i].parse().unwrap();

    //                     for j in 0..seed_range {
    //                         seeds.push(seed_from + j);
    //                     }
                        
    //                 }
    //                 continue;
    //             }
    //             if l == "" {
    //                 continue;
    //             }
    //             if l.contains("map:") {
    //                 maps.push(Map{from_elems: vec![], to_elems_offset: vec![]});
    //                 continue;
    //             }
    //             let splitl: Vec<&str> = l.trim().split(' ').collect();

    //             let to: i128 = splitl[0].parse().unwrap();
    //             let from: i128 = splitl[1].parse().unwrap();
    //             let length: i128 = splitl[2].parse().unwrap();

    //             let mlen = maps.len();
    //             maps[mlen - 1].add_elements(MapElement {min: from, max: from + length - 1}, to - from );
                

    //         }
    //     }
    // }
    // for seed in seeds {
    //     let mut seedn = seed;
    //     for map in &maps {
    //         seedn = map.translate(seedn);
    //     }
    //     min_loc = cmp::min(min_loc, seedn);
    // }
    // println!("part2: {min_loc}")

}


// output is wrapped in a Result
// returns an Iterator to he Reader of lines of the file
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}


