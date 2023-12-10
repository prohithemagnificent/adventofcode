use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::time::{Duration, Instant};
use std::collections::HashMap;
use regex::Regex;
use num;


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
    let mut map: HashMap<String, (String, String)> = HashMap::new();
    let mut sequence: Vec<char> = vec![];
    if let Ok(lines) = read_lines(r"E:\rusty\adventofcode\2023\day08\input.txt") {
        for line in lines {
            if let Ok(l) = line {
                if l.contains("LRLR") {
                    sequence = l.chars().collect();
                }
                if l.contains("=") {
                    let re = Regex::new(r"([A-Z]{3}) = \(([A-Z]{3}), ([A-Z]{3})\)").unwrap();
                    let caps = re.captures(&l).unwrap();
                    map.insert(caps[1].to_string(), (caps[2].to_string(), caps[3].to_string()));
                }

            }
        }
    }
    // println!("{:?}", sequence);
    let mut i: usize = 0;
    let mut elem: String = "AAA".to_string();
    while elem != "ZZZ" {
        let lr = sequence[i % sequence.len()];
        let (l, r) = &map[&elem];
        if lr == 'L' {
            elem = l.to_string();
        } else {
            elem = r.to_string();
        }
        i += 1;
    }

    println!("part 1: {i}");


}



fn finished(elems: Vec<String>) -> bool {
    let mut ret = true;
    for s in elems {
        if !s.ends_with("Z") {
            ret = false;
            break;
        }
    }
    return ret;
}

fn finishedel(elem: &String) -> bool {
    let mut ret = true;
    if !elem.ends_with("Z") {
            ret = false;
    }
    return ret;
}


fn part_two() {
    let mut map: HashMap<String, (String, String)> = HashMap::new();
    let mut sequence: Vec<char> = vec![];
    let mut elems: Vec<String> = vec![];
    if let Ok(lines) = read_lines(r"E:\rusty\adventofcode\2023\day08\input.txt") {
    // if let Ok(lines) = read_lines(r"E:\rusty\adventofcode\2023\day08\test.txt") {
        for line in lines {
            if let Ok(l) = line {
                if l.contains("LRLR") {
                    sequence = l.chars().collect();
                }
                if l.contains("=") {
                    let re = Regex::new(r"([A-Z]{3}) = \(([A-Z]{3}), ([A-Z]{3})\)").unwrap();
                    // let re = Regex::new(r"([0-9A-Z]{3}) = \(([0-9A-Z]{3}), ([0-9A-Z]{3})\)").unwrap();   // for test input
                    let caps = re.captures(&l).unwrap();
                    map.insert(caps[1].to_string(), (caps[2].to_string(), caps[3].to_string()));

                    if caps[1].to_string().ends_with("A") {
                        elems.push(caps[1].to_string());
                    }
                }

            }
        }
    }
    // println!("{:?}", sequence);
    let mut i: usize = 0;

    // brute force too long
    // while !finished(elems.clone()) {
    //     let lr = sequence[i % sequence.len()];
    //     for elem in elems.iter_mut() {
    //         let (l, r) = &map[elem];
    //         if lr == 'L' {
    //             *elem = l.to_string();
    //         } else {
    //             *elem = r.to_string();
    //         }
    //     }
    //     i += 1;
    //     if i % 1_000_000 == 0 {
    //         println!("{}", i);
    //     }
    // }

    let mut steps: Vec<u128> = vec![];
    for elem in elems.iter_mut() {
        i = 0;
        while !finishedel(elem) {
            let lr = sequence[i % sequence.len()];
            let (l, r) = &map[elem];
            if lr == 'L' {
                *elem = l.to_string();
            } else {
                *elem = r.to_string();
            }
        
            i += 1;
            if i % 1_000_000 == 0 {
                println!("{}", i);
            }
        }
        steps.push(i as u128); 
    }
    assert!(steps.len() > 0);
    let mut lcm = steps[0];
    for x in 1..steps.len() {
        lcm = num::integer::lcm(lcm, steps[x])
    }

    println!("part 2: {lcm}");

}



// output is wrapped in a Result
// returns an Iterator to he Reader of lines of the file
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}




// --- Day 8: Haunted Wasteland ---
// You're still riding a camel across Desert Island when you spot a sandstorm quickly approaching. When you turn to warn the Elf, she disappears before your eyes! To be fair, she had just finished warning you about ghosts a few minutes ago.

// One of the camel's pouches is labeled "maps" - sure enough, it's full of documents (your puzzle input) about how to navigate the desert. At least, you're pretty sure that's what they are; one of the documents contains a list of left/right instructions, and the rest of the documents seem to describe some kind of network of labeled nodes.

// It seems like you're meant to use the left/right instructions to navigate the network. Perhaps if you have the camel follow the same instructions, you can escape the haunted wasteland!

// After examining the maps for a bit, two nodes stick out: AAA and ZZZ. You feel like AAA is where you are now, and you have to follow the left/right instructions until you reach ZZZ.

// This format defines each node of the network individually. For example:

// RL

// AAA = (BBB, CCC)
// BBB = (DDD, EEE)
// CCC = (ZZZ, GGG)
// DDD = (DDD, DDD)
// EEE = (EEE, EEE)
// GGG = (GGG, GGG)
// ZZZ = (ZZZ, ZZZ)
// Starting with AAA, you need to look up the next element based on the next left/right instruction in your input. In this example, start with AAA and go right (R) by choosing the right element of AAA, CCC. Then, L means to choose the left element of CCC, ZZZ. By following the left/right instructions, you reach ZZZ in 2 steps.

// Of course, you might not find ZZZ right away. If you run out of left/right instructions, repeat the whole sequence of instructions as necessary: RL really means RLRLRLRLRLRLRLRL... and so on. For example, here is a situation that takes 6 steps to reach ZZZ:

// LLR

// AAA = (BBB, BBB)
// BBB = (AAA, ZZZ)
// ZZZ = (ZZZ, ZZZ)
// Starting at AAA, follow the left/right instructions. How many steps are required to reach ZZZ?

// Your puzzle answer was 19637.

// --- Part Two ---
// The sandstorm is upon you and you aren't any closer to escaping the wasteland. You had the camel follow the instructions, but you've barely left your starting position. It's going to take significantly more steps to escape!

// What if the map isn't for people - what if the map is for ghosts? Are ghosts even bound by the laws of spacetime? Only one way to find out.

// After examining the maps a bit longer, your attention is drawn to a curious fact: the number of nodes with names ending in A is equal to the number ending in Z! If you were a ghost, you'd probably just start at every node that ends with A and follow all of the paths at the same time until they all simultaneously end up at nodes that end with Z.

// For example:

// LR

// 11A = (11B, XXX)
// 11B = (XXX, 11Z)
// 11Z = (11B, XXX)
// 22A = (22B, XXX)
// 22B = (22C, 22C)
// 22C = (22Z, 22Z)
// 22Z = (22B, 22B)
// XXX = (XXX, XXX)
// Here, there are two starting nodes, 11A and 22A (because they both end with A). As you follow each left/right instruction, use that instruction to simultaneously navigate away from both nodes you're currently on. Repeat this process until all of the nodes you're currently on end with Z. (If only some of the nodes you're on end with Z, they act like any other node and you continue as normal.) In this example, you would proceed as follows:

// Step 0: You are at 11A and 22A.
// Step 1: You choose all of the left paths, leading you to 11B and 22B.
// Step 2: You choose all of the right paths, leading you to 11Z and 22C.
// Step 3: You choose all of the left paths, leading you to 11B and 22Z.
// Step 4: You choose all of the right paths, leading you to 11Z and 22B.
// Step 5: You choose all of the left paths, leading you to 11B and 22C.
// Step 6: You choose all of the right paths, leading you to 11Z and 22Z.
// So, in this example, you end up entirely on nodes that end in Z after 6 steps.

// Simultaneously start on every node that ends with A. How many steps does it take before you're only on nodes that end with Z?

// Your puzzle answer was 8811050362409.

// Both parts of this puzzle are complete! They provide two gold stars: **