use std::collections::HashSet;
use std::collections::VecDeque;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::time::{Duration, Instant};


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

// beam element arriving in a pos (r,c) with direction dir
// spawns new beams
#[derive(Eq, Hash, PartialEq, Copy, Clone, Debug)]
struct Beam {
    r: usize,  // top to bottom
    c: usize,   // left to right
    dir: char   // U D L R 
}

impl Beam {
    fn advance(&self, map: &Vec<Vec<char>>, height: usize, width: usize) -> Vec<Beam> {
        let mut new_beams: Vec<Beam> = Vec::new();
        // ------------------------------------------
        // ------------------------------------------
        if map[self.r][self.c] == '.' {
            if        self.dir == 'U' && self.r > 0 {
                new_beams.push(Beam{r: self.r - 1, c: self.c, dir: self.dir})
            } else if self.dir == 'D' && self.r < height - 1 {
                new_beams.push(Beam{r: self.r + 1, c: self.c, dir: self.dir})
            } else if self.dir == 'L' && self.c > 0 {
                new_beams.push(Beam{r: self.r , c: self.c - 1, dir: self.dir})
            } else if self.dir == 'R' && self.c < width - 1 {
                new_beams.push(Beam{r: self.r, c: self.c + 1, dir: self.dir})
            }
        // ------------------------------------------
        // ------------------------------------------
        } else if map[self.r][self.c] == '/' {
            if        self.dir == 'U' && self.c < width - 1 {
                new_beams.push(Beam{r: self.r , c: self.c + 1, dir: 'R'})
            } else if self.dir == 'D' && self.c > 0 {
                new_beams.push(Beam{r: self.r , c: self.c - 1, dir: 'L'})
            } else if self.dir == 'L' && self.r < height - 1 {
                new_beams.push(Beam{r: self.r + 1 , c: self.c, dir: 'D'})
            } else if self.dir == 'R' && self.r > 0 {
                new_beams.push(Beam{r: self.r - 1, c: self.c, dir: 'U'})
            }
        // ------------------------------------------
        // ------------------------------------------
        } else if map[self.r][self.c] == '\\' {
            if        self.dir == 'U' && self.c > 0 {
                new_beams.push(Beam{r: self.r , c: self.c - 1, dir: 'L'})
            } else if self.dir == 'D' && self.c < width - 1 {
                new_beams.push(Beam{r: self.r , c: self.c + 1, dir: 'R'})
            } else if self.dir == 'L' && self.r > 0 {
                new_beams.push(Beam{r: self.r - 1, c: self.c, dir: 'U'})
            } else if self.dir == 'R' && self.r < height - 1 {
                new_beams.push(Beam{r: self.r + 1, c: self.c, dir: 'D'})
            }
        // ------------------------------------------
        // ------------------------------------------
        } else if map[self.r][self.c] == '|' {
            if        self.dir == 'U' && self.r > 0 {
                new_beams.push(Beam{r: self.r - 1, c: self.c, dir: self.dir})
            } else if self.dir == 'D' && self.r < height - 1 {
                new_beams.push(Beam{r: self.r + 1, c: self.c, dir: self.dir})
            } else if self.dir == 'L' || self.dir == 'R' {
                if self.r > 0 {
                    new_beams.push(Beam{r: self.r - 1, c: self.c, dir: 'U'})
                }
                if self.r < height - 1 {
                    new_beams.push(Beam{r: self.r + 1, c: self.c, dir: 'D'})
                }
            }
        // ------------------------------------------
        // ------------------------------------------
        } else if map[self.r][self.c] == '-' {
            if        self.dir == 'L' && self.c > 0 {
                new_beams.push(Beam{r: self.r , c: self.c - 1, dir: self.dir})
            } else if self.dir == 'R' && self.c < width - 1 {
                new_beams.push(Beam{r: self.r, c: self.c + 1, dir: self.dir})
            } else if self.dir == 'U' || self.dir == 'D' {
                if self.c > 0 {
                    new_beams.push(Beam{r: self.r, c: self.c - 1, dir: 'L'})
                }
                if self.c < height - 1 {
                    new_beams.push(Beam{r: self.r, c: self.c + 1, dir: 'R'})
                }
            }
        }
        return new_beams;
    }
}



fn part_one() {
    let mut map: Vec<Vec<char>> = Vec::new();
    if let Ok(lines) = read_lines(r"E:\rusty\adventofcode\2023\day16\input.txt") {
        for line in lines {
            if let Ok(l) = line {
                map.push(l.trim().chars().collect());
            }
        }
    }
    // let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut beampit: HashSet<Beam> = HashSet::new();
    let height = map.len();
    assert!(height > 0);
    let width = map[0].len();
    assert!(width > 0);


    let mut dq: VecDeque<Beam> = VecDeque::new();
    let initialbeam = Beam{r: 0, c: 0, dir: 'R'};
    dq.push_back(initialbeam);
    beampit.insert(initialbeam);

    while !dq.is_empty() {
        let b: Beam = dq.pop_front().unwrap();
        // println!("{:?}", b);
        let new_beams: Vec<Beam> = b.advance(&map, height, width);
        for new_beam in new_beams {
            if beampit.insert(new_beam) {
                //  println!("new beam: {:?}", new_beam);
                dq.push_back(new_beam);
            }
        }
    }

    let mut positions: HashSet<(usize, usize)> = HashSet::new();
    for beam in beampit {
        positions.insert((beam.r, beam.c));
    }
    println!("part 1: {}", positions.len());    
}




fn part_two() {
    let mut map: Vec<Vec<char>> = Vec::new();
    if let Ok(lines) = read_lines(r"E:\rusty\adventofcode\2023\day16\input.txt") {
        for line in lines {
            if let Ok(l) = line {
                map.push(l.trim().chars().collect());
            }
        }
    }
    let height = map.len();
    assert!(height > 0);
    let width = map[0].len();
    assert!(width > 0);
    let mut maxfill = 0;
    let mut initials: Vec<Beam> = Vec::new();


    for i in 0..height {
        initials.push(Beam{r: i, c: 0, dir: 'R'});
        initials.push(Beam{r: i, c: width - 1, dir: 'L'});
    }
    for i in 0..width {
        initials.push(Beam{r: 0, c: i, dir: 'D'});
        initials.push(Beam{r: height - 1, c: i, dir: 'U'});
    }


    for initialbeam in initials {
        // let mut visited: HashSet<(usize, usize)> = HashSet::new();
        let mut beampit: HashSet<Beam> = HashSet::new();

        let mut dq: VecDeque<Beam> = VecDeque::new();
        dq.push_back(initialbeam);
        beampit.insert(initialbeam);

        while !dq.is_empty() {
            let b: Beam = dq.pop_front().unwrap();
            // println!("{:?}", b);
            let new_beams: Vec<Beam> = b.advance(&map, height, width);
            for new_beam in new_beams {
                if beampit.insert(new_beam) {
                    //  println!("new beam: {:?}", new_beam);
                    dq.push_back(new_beam);
                }
            }
        }

        let mut positions: HashSet<(usize, usize)> = HashSet::new();
        for beam in beampit {
            positions.insert((beam.r, beam.c));
        }
        maxfill = usize::max(positions.len(), maxfill);
    }

    println!("part 2: {}", maxfill);
    
}

// part 1: 6883
// part 1 elapsed time: 0.04842
// part 2: 7228
// part 2 elapsed time: 5.04753

// --- Day 16: The Floor Will Be Lava ---
// With the beam of light completely focused somewhere, the reindeer leads you deeper still into the Lava Production Facility. At some point, you realize that the steel facility walls have been replaced with cave, and the doorways are just cave, and the floor is cave, and you're pretty sure this is actually just a giant cave.

// Finally, as you approach what must be the heart of the mountain, you see a bright light in a cavern up ahead. There, you discover that the beam of light you so carefully focused is emerging from the cavern wall closest to the facility and pouring all of its energy into a contraption on the opposite side.

// Upon closer inspection, the contraption appears to be a flat, two-dimensional square grid containing empty space (.), mirrors (/ and \), and splitters (| and -).

// The contraption is aligned so that most of the beam bounces around the grid, but each tile on the grid converts some of the beam's light into heat to melt the rock in the cavern.

// You note the layout of the contraption (your puzzle input). For example:

// .|...\....
// |.-.\.....
// .....|-...
// ........|.
// ..........
// .........\
// ..../.\\..
// .-.-/..|..
// .|....-|.\
// ..//.|....
// The beam enters in the top-left corner from the left and heading to the right. Then, its behavior depends on what it encounters as it moves:

// If the beam encounters empty space (.), it continues in the same direction.
// If the beam encounters a mirror (/ or \), the beam is reflected 90 degrees depending on the angle of the mirror. For instance, a rightward-moving beam that encounters a / mirror would continue upward in the mirror's column, while a rightward-moving beam that encounters a \ mirror would continue downward from the mirror's column.
// If the beam encounters the pointy end of a splitter (| or -), the beam passes through the splitter as if the splitter were empty space. For instance, a rightward-moving beam that encounters a - splitter would continue in the same direction.
// If the beam encounters the flat side of a splitter (| or -), the beam is split into two beams going in each of the two directions the splitter's pointy ends are pointing. For instance, a rightward-moving beam that encounters a | splitter would split into two beams: one that continues upward from the splitter's column and one that continues downward from the splitter's column.
// Beams do not interact with other beams; a tile can have many beams passing through it at the same time. A tile is energized if that tile has at least one beam pass through it, reflect in it, or split in it.

// In the above example, here is how the beam of light bounces around the contraption:

// >|<<<\....
// |v-.\^....
// .v...|->>>
// .v...v^.|.
// .v...v^...
// .v...v^..\
// .v../2\\..
// <->-/vv|..
// .|<<<2-|.\
// .v//.|.v..
// Beams are only shown on empty tiles; arrows indicate the direction of the beams. If a tile contains beams moving in multiple directions, the number of distinct directions is shown instead. Here is the same diagram but instead only showing whether a tile is energized (#) or not (.):

// ######....
// .#...#....
// .#...#####
// .#...##...
// .#...##...
// .#...##...
// .#..####..
// ########..
// .#######..
// .#...#.#..
// Ultimately, in this example, 46 tiles become energized.

// The light isn't energizing enough tiles to produce lava; to debug the contraption, you need to start by analyzing the current situation. With the beam starting in the top-left heading right, how many tiles end up being energized?

// Your puzzle answer was 6883.

// --- Part Two ---
// As you try to work out what might be wrong, the reindeer tugs on your shirt and leads you to a nearby control panel. There, a collection of buttons lets you align the contraption so that the beam enters from any edge tile and heading away from that edge. (You can choose either of two directions for the beam if it starts on a corner; for instance, if the beam starts in the bottom-right corner, it can start heading either left or upward.)

// So, the beam could start on any tile in the top row (heading downward), any tile in the bottom row (heading upward), any tile in the leftmost column (heading right), or any tile in the rightmost column (heading left). To produce lava, you need to find the configuration that energizes as many tiles as possible.

// In the above example, this can be achieved by starting the beam in the fourth tile from the left in the top row:

// .|<2<\....
// |v-v\^....
// .v.v.|->>>
// .v.v.v^.|.
// .v.v.v^...
// .v.v.v^..\
// .v.v/2\\..
// <-2-/vv|..
// .|<<<2-|.\
// .v//.|.v..
// Using this configuration, 51 tiles are energized:

// .#####....
// .#.#.#....
// .#.#.#####
// .#.#.##...
// .#.#.##...
// .#.#.##...
// .#.#####..
// ########..
// .#######..
// .#...#.#..
// Find the initial beam configuration that energizes the largest number of tiles; how many tiles are energized in that configuration?

// Your puzzle answer was 7228.

// Both parts of this puzzle are complete! They provide two gold stars: **



// output is wrapped in a Result
// returns an Iterator to he Reader of lines of the file
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}




