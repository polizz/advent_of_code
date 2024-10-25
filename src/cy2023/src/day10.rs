// --- Day 10: Pipe Maze ---
// You use the hang glider to ride the hot air from Desert Island all the way up to the floating metal island. This island is surprisingly cold and there definitely aren't any thermals to glide on, so you leave your hang glider behind.
//
// You wander around for a while, but you don't find any people or animals. However, you do occasionally find signposts labeled "Hot Springs" pointing in a seemingly consistent direction; maybe you can find someone at the hot springs and ask them where the desert-machine parts are made.
//
// The landscape here is alien; even the flowers and trees are made of metal. As you stop to admire some metal grass, you notice something metallic scurry away in your peripheral vision and jump into a big pipe! It didn't look like any animal you've ever seen; if you want a better look, you'll need to get ahead of it.
//
// Scanning the area, you discover that the entire field you're standing on is densely packed with pipes; it was hard to tell at first because they're the same metallic silver color as the "ground". You make a quick sketch of all of the surface pipes you can see (your puzzle input).
//
// The pipes are arranged in a two-dimensional grid of tiles:
//
// | is a vertical pipe connecting north and south.
// - is a horizontal pipe connecting east and west.
// L is a 90-degree bend connecting north and east.
// J is a 90-degree bend connecting north and west.
// 7 is a 90-degree bend connecting south and west.
// F is a 90-degree bend connecting south and east.
// . is ground; there is no pipe in this tile.
// S is the starting position of the animal; there is a pipe on this tile, but your sketch doesn't show what shape the pipe has.
// Based on the acoustics of the animal's scurrying, you're confident the pipe that contains the animal is one large, continuous loop.
//
// For example, here is a square loop of pipe:
//
// .....
// .F-7.
// .|.|.
// .L-J.
// .....
// If the animal had entered this loop in the northwest corner, the sketch would instead look like this:
//
// .....
// .S-7.
// .|.|.
// .L-J.
// .....
// In the above diagram, the S tile is still a 90-degree F bend: you can tell because of how the adjacent pipes connect to it.
//
// Unfortunately, there are also many pipes that aren't connected to the loop! This sketch shows the same loop as above:
//
// -L|F7
// 7S-7|
// L|7||
// -L-J|
// L|-JF
// In the above diagram, you can still figure out which pipes form the main loop: they're the ones connected to S, pipes those pipes connect to, pipes those pipes connect to, and so on. Every pipe in the main loop connects to its two neighbors (including S, which will have exactly two pipes connecting to it, and which is assumed to connect back to those two pipes).
//
// Here is a sketch that contains a slightly more complex main loop:
//
// ..F7.
// .FJ|.
// SJ.L7
// |F--J
// LJ...
// Here's the same example sketch with the extra, non-main-loop pipe tiles also shown:
//
// 7-F7-
// .FJ|7
// SJLL7
// |F--J
// LJ.LJ
// If you want to get out ahead of the animal, you should find the tile in the loop that is farthest from the starting position. Because the animal is in the pipe, it doesn't make sense to measure this by direct distance. Instead, you need to find the tile that would take the longest number of steps along the loop to reach from the starting point - regardless of which way around the loop the animal went.
//
// In the first example with the square loop:
//
// .....
// .S-7.
// .|.|.
// .L-J.
// .....
// You can count the distance each tile in the loop is from the starting point like this:
//
// .....
// .012.
// .1.3.
// .234.
// .....
// In this example, the farthest point from the start is 4 steps away.
//
// Here's the more complex loop again:
//
//*************
// ..F7.
// .FJ|.
// SJ.L7
// |F--J
// LJ...
//*************
// Here are the distances for each tile on that loop:
//
//
//
//
// ..45.
// .236.
// 01.78
// 14567
// 23...
// Find the single giant loop starting at S. How many steps along the loop does it take to get from the starting position to the point farthest from the starting position?
//

//**--- Part Two ---**
// You quickly reach the farthest point of the loop, but the animal never emerges. Maybe its nest is within the area enclosed by the loop?
//
// To determine whether it's even worth taking the time to search for such a nest, you should calculate how many tiles are contained within the loop. For example:
//
// ...........
// .S-------7.
// .|F-----7|.
// .||.....||.
// .||.....||.
// .|L-7.F-J|.
// .|..|.|..|.
// .L--J.L--J.
// ...........
// The above loop encloses merely four tiles - the two pairs of . in the southwest and southeast (marked I below). The middle . tiles (marked O below) are not in the loop. Here is the same loop again with those regions marked:
//
// ...........
// .S-------7.
// .|F-----7|.
// .||OOOOO||.
// .||OOOOO||.
// .|L-7OF-J|.
// .|II|O|II|.
// .L--JOL--J.
// .....O.....
// In fact, there doesn't even need to be a full tile path to the outside for tiles to count as outside the loop - squeezing between pipes is also allowed! Here, I is still within the loop and O is still outside the loop:
//
// ..........
// .S------7.
// .|F----7|.
// .||OOOO||.
// .||OOOO||.
// .|L-7F-J|.
// .|II||II|.
// .L--JL--J.
// ..........
// In both of the above examples, 4 tiles are enclosed by the loop.
//
// Here's a larger example:
//
// .F----7F7F7F7F-7....
// .|F--7||||||||FJ....
// .||.FJ||||||||L7....
// FJL7L7LJLJ||LJ.L-7..
// L--J.L7...LJS7F-7L7.
// ....F-J..F7FJ|L7L7L7
// ....L7.F7||L7|.L7L7|
// .....|FJLJ|FJ|F7|.LJ
// ....FJL-7.||.||||...
// ....L---J.LJ.LJLJ...
// The above sketch has many random bits of ground, some of which are in the loop (I) and some of which are outside it (O):
//
// OF----7F7F7F7F-7OOOO
// O|F--7||||||||FJOOOO
// O||OFJ||||||||L7OOOO
// FJL7L7LJLJ||LJIL-7OO
// L--JOL7IIILJS7F-7L7O
// OOOOF-JIIF7FJ|L7L7L7
// OOOOL7IF7||L7|IL7L7|
// OOOOO|FJLJ|FJ|F7|OLJ
// OOOOFJL-7O||O||||OOO
// OOOOL---JOLJOLJLJOOO
// In this larger example, 8 tiles are enclosed by the loop.
//
// Any tile that isn't part of the main loop can count as being enclosed by the loop. Here's another example with many bits of junk pipe lying around that aren't connected to the main loop at all:
//
// FF7FSF7F7F7F7F7F---7
// L|LJ||||||||||||F--J
// FL-7LJLJ||||||LJL-77
// F--JF--7||LJLJ7F7FJ-
// L---JF-JLJ.||-FJLJJ7
// |F|F-JF---7F7-L7L|7|
// |FFJF7L7F-JF7|JL---7
// 7-L-JL7||F7|L7F-7F7|
// L.L7LFJ|||||FJL7||LJ
// L7JLJL-JLJLJL--JLJ.L
// Here are just the tiles that are enclosed by the loop marked with I:
//
//
// |F7FSF7F7F7F7F7F---7
// ||LJ||||||||||||F--J
// |L-7LJLJ||||||LJL-7|
// F--JF--7||LJLJ·F7FJ|
// L---JF-JLJ····FJLJ |
// |  F-JF---7···L7   |
// | FJF7L7F-JF7··L---7
// | L-JL7||F7|L7F-7F7|
// |    FJ|||||FJL7||LJ
// |____L-JLJLJL--JLJ_|
//
//
//
//
//
// FF7FSF7F7F7F7F7F---7
// L|LJ||||||||||||F--J
// FL-7LJLJ||||||LJL-77
// F--JF--7||LJLJIF7FJ-
// L---JF-JLJIIIIFJLJJ7
// |F|F-JF---7IIIL7L|7|
// |FFJF7L7F-JF7IIL---7
// 7-L-JL7||F7|L7F-7F7|
// L.L7LFJ|||||FJL7||LJ
// L7JLJL-JLJLJL--JLJ.L
// In this last example, 10 tiles are enclosed by the loop.
//
// Figure out whether you have time to search for the nest by calculating the area within the loop. How many tiles are enclosed by the loop?

use std::collections::HashSet;

#[derive(Debug, Copy, Clone)]
enum Dir {
    Down,
    Up,
    Left,
    Right,
}

fn get_start_pipe_type(pipes: &Vec<bool>, map: &[u8], start_pos: usize, width: usize) -> u8 {
    let rt = if start_pos + 1 < map.len() - 1 {
        Some(start_pos + 1)
    } else {
        None
    };

    let lt = start_pos.checked_sub(1);
    let up = start_pos.checked_sub(width + 1);
    let dn = if start_pos + width + 1 < map.len() {
        Some(start_pos + width + 1)
    } else {
        None
    };

    // which tiles are pointing at S?
    let mut start_type = 0u8;

    unsafe {
        if up.is_some_and(|up| {
            *pipes.get_unchecked(up) && (map[up] == b'|' || map[up] == b'F' || map[up] == b'7')
        }) {
            // above
            start_type |= 0x1;
        }
        if rt.is_some_and(|rt| {
            *pipes.get_unchecked(rt) && (map[rt] == b'7' || map[rt] == b'-' || map[rt] == b'J')
        }) {
            // right
            start_type |= 0x2;
        }
        if dn.is_some_and(|dn| {
            *pipes.get_unchecked(dn) && (map[dn] == b'|' || map[dn] == b'L' || map[dn] == b'J')
        }) {
            // down
            start_type |= 0x4;
        }
        if lt.is_some_and(|lt| {
            *pipes.get_unchecked(lt) && (map[lt] == b'F' || map[lt] == b'-' || map[lt] == b'L')
        }) {
            // left
            start_type |= 0x8;
        }
    }

    // above and right
    if start_type & 0x3 == 0x3 {
        return b'L';
    }
    // above and down
    else if start_type & 0x5 == 0x5 {
        return b'|';
    }
    // above and left
    else if start_type & 0x9 == 0x9 {
        return b'J';
    }
    // right and down
    else if start_type & 0x6 == 0x6 {
        return b'F';
    }
    // right and left
    else if start_type & 0xA == 0xA {
        return b'-';
    }
    // left and down
    else if start_type & 0xC == 0xC {
        return b'7';
    } else {
        panic!("not good")
    }
}

pub fn solve2(map: &mut [u8]) -> usize {
    let start = map.iter().position(|&c| c == b'S').unwrap();
    let width = map.iter().position(|&c| c == b'\n').unwrap();

    let (mut pos, mut dir) = if matches!(map[start + width + 1], b'|' | b'L' | b'J') {
        (start + width + 1, Dir::Down)
    } else if matches!(map[start - width - 1], b'|' | b'F' | b'7') {
        (start - width - 1, Dir::Up)
    } else {
        // must be left and right for a loop to exist if up and down were not found
        (start - 1, Dir::Left)
    };

    use std::str;

    println!("S:{:?}, width:{:?}", &start, &width);
    println!("start: {:?}", &(str::from_utf8(&[map[pos]]).unwrap(), dir));

    let mut pipes = vec![false; map.len()];

    // from starting S, follow all pipe connections, marking them as pipes in "pipes" vec
    let _n = std::iter::repeat(())
        .position(|_| unsafe {
            *pipes.get_unchecked_mut(pos) = true;
            match (map[pos], dir) {
                (b'|', Dir::Down) => pos += width + 1,
                (b'|', Dir::Up) => pos -= width + 1,
                (b'-', Dir::Right) => pos += 1,
                (b'-', Dir::Left) => pos -= 1,
                (b'J', Dir::Right) | (b'L', Dir::Left) => {
                    pos -= width + 1;
                    dir = Dir::Up;
                }
                (b'7', Dir::Up) | (b'J', Dir::Down) => {
                    pos -= 1;
                    dir = Dir::Left;
                }
                (b'F', Dir::Up) | (b'L', Dir::Down) => {
                    pos += 1;
                    dir = Dir::Right;
                }
                (b'7', Dir::Right) | (b'F', Dir::Left) => {
                    pos += width + 1;
                    dir = Dir::Down;
                }
                (b'S', _) => return true,
                _ => unreachable!(),
            };
            false
        })
        .unwrap();

    let start_effective_pipe_type = get_start_pipe_type(&pipes, map, start, width);
    unsafe {
        *map.get_unchecked_mut(start) = start_effective_pipe_type;
    }

    #[derive(Clone, Debug)]
    enum Start {
        L,
        F,
    }

    impl Mode {
        fn toggle(&self) -> Self {
            match self {
                Mode::Inside => Mode::Outside,
                Mode::Outside => Mode::Inside,
            }
        }
    }
    #[derive(Clone, Debug)]
    enum Mode {
        Inside,
        Outside,
    }

    #[derive(Clone, Debug)]
    enum State {
        Barrier(Start, Mode),
        Area(Mode),
    }

    let mut state = State::Area(Mode::Outside);
    let inside_pos = map
        .iter()
        .enumerate()
        .filter(|(pos, tile)| unsafe {
            let is_pipe = *pipes.get_unchecked(*pos);

            match &state {
                State::Area(m) => {
                    if is_pipe {
                        if matches!(tile, b'|') {
                            state = State::Area(m.toggle());
                        } else if matches!(tile, b'L') {
                            state = State::Barrier(Start::L, m.clone());
                        } else if matches!(tile, b'F') {
                            state = State::Barrier(Start::F, m.clone());
                        }
                        return false;
                    } else {
                        // if inside, return true to count area
                        return matches!(m, Mode::Inside);
                    }
                }
                State::Barrier(s, m) => {
                    if is_pipe {
                        match s {
                            Start::F => {
                                if matches!(tile, b'J') {
                                    state = State::Area(m.toggle());
                                } else if matches!(tile, b'7') {
                                    state = State::Area(m.clone());
                                }
                            }
                            Start::L => {
                                if matches!(tile, b'7') {
                                    state = State::Area(m.toggle());
                                } else if matches!(tile, b'J') {
                                    state = State::Area(m.clone());
                                }
                            }
                        }
                    }
                    return false;
                }
            }
        })
        // .inspect(|x| println!("x: {:?}", &x))
        .collect::<Vec<(usize, &u8)>>();

    let inside_area = inside_pos.len();
    println!("Inside area: {}", inside_area);

    print_map(width, map, &pipes, &inside_pos);

    inside_area
}

pub fn print_map(width: usize, map: &[u8], pipes: &Vec<bool>, inside_tiles: &Vec<(usize, &u8)>) {
    let inside_hash: HashSet<&usize> = HashSet::from_iter(inside_tiles.iter().map(|(t, _)| t));

    std::iter::repeat("^")
        .take(width)
        .for_each(|c| print!("{}", c));
    println!();

    pipes
        .iter()
        .zip(map)
        .enumerate()
        .for_each(|(pos, (is_pipe, tile))| {
            let output = if *is_pipe {
                *tile
            } else {
                if inside_hash.contains(&pos) {
                    183
                } else if *tile == b'\n' {
                    b'\n'
                } else {
                    160
                }
            };

            match output {
                b'\n' => println!("|"),
                _ => print!("{}", output as char),
            }
        });

    println!();

    std::iter::repeat("_")
        .take(width)
        .for_each(|c| print!("{}", c));

    println!();
}

pub fn solve(map: &[u8]) -> usize {
    let start = map.iter().position(|&c| c == b'S').unwrap();
    let width = map.iter().position(|&c| c == b'\n').unwrap();

    let (mut pos, mut dir) = if matches!(map[start + width + 1], b'|' | b'L' | b'J') {
        (start + width + 1, Dir::Down)
    } else if matches!(map[start - width - 1], b'|' | b'F' | b'7') {
        (start - width - 1, Dir::Up)
    } else {
        // must be left and right for a loop to exist if up and down were not found
        (start - 1, Dir::Left)
    };

    use std::str;

    println!("S:{:?}, width:{:?}", &start, &width);
    println!("start: {:?}", &(str::from_utf8(&[map[pos]]).unwrap(), dir));

    (1 + std::iter::repeat(())
        .position(|_| {
            // println!("{:?}", &(str::from_utf8(&[map[pos]]).unwrap(), dir));
            match (map[pos], dir) {
                (b'|', Dir::Down) => pos += width + 1,
                (b'|', Dir::Up) => pos -= width + 1,
                (b'-', Dir::Right) => pos += 1,
                (b'-', Dir::Left) => pos -= 1,
                (b'J', Dir::Right) | (b'L', Dir::Left) => {
                    pos -= width + 1;
                    dir = Dir::Up;
                }
                (b'7', Dir::Up) | (b'J', Dir::Down) => {
                    pos -= 1;
                    dir = Dir::Left;
                }
                (b'F', Dir::Up) | (b'L', Dir::Down) => {
                    pos += 1;
                    dir = Dir::Right;
                }
                (b'7', Dir::Right) | (b'F', Dir::Left) => {
                    pos += width + 1;
                    dir = Dir::Down;
                }
                (b'S', _) => return true,
                _ => unreachable!(),
            };
            false
        })
        .unwrap())
        / 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day10_2_custom1() {
        let start = std::time::Instant::now();

        let mut input = r#"...........
.S-------7.
.|F-----7|.
.||.....||.
FJ|.....||.
|.L-7.F-J|.
|...|.|..|.
|...|.L--J.
L---J......"#
            .to_owned();

        let input = unsafe { input.as_bytes_mut() };

        use std::str;
        println!("{:?}", str::from_utf8(&input));
        let area = solve2(input);

        assert_eq!(area, 9usize);
        println!("Process in: {:?}", start.elapsed());
    }

    #[test]
    fn day10_2_file() {
        let start = std::time::Instant::now();
        let mut input = include_bytes!("fixtures/day10.txt").clone();

        // let area = other_solve(input);
        let area = solve2(input.as_mut_slice());

        assert_eq!(area, 325usize);
        println!("Process in: {:?}", start.elapsed());
    }

    #[test]
    fn day10_2_sample4() {
        let start = std::time::Instant::now();
        let mut input = r#"FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L"#
            .to_owned();
        let input = unsafe { input.as_bytes_mut() };

        // use std::str;
        // println!("{:?}", str::from_utf8(&input));
        let area = solve2(input);

        assert_eq!(area, 10usize);
        println!("Process in: {:?}", start.elapsed());
    }

    #[test]
    fn day10_2_sample3() {
        let start = std::time::Instant::now();
        let mut input = r#".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ..."#
            .to_owned();

        let input = unsafe { input.as_bytes_mut() };

        // use std::str;
        // println!("{:?}", str::from_utf8(&input));
        let area = solve2(input);

        assert_eq!(area, 8usize);
        println!("Process in: {:?}", start.elapsed());
    }

    #[test]
    fn day10_2_sample2() {
        let start = std::time::Instant::now();

        let mut input = r#"...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
..........."#
            .to_owned();
        let input = unsafe { input.as_bytes_mut() };

        use std::str;
        println!("{:?}", str::from_utf8(&input));
        let area = solve2(input);

        assert_eq!(area, 4usize);
        println!("Process in: {:?}", start.elapsed());
    }

    #[test]
    fn day10_file1() {
        let start = std::time::Instant::now();
        let input = include_bytes!("fixtures/day10.txt");

        let furthest_tile = solve(input);

        assert_eq!(furthest_tile, 6846usize);
        println!("Process in: {:?}", start.elapsed());
    }

    #[test]
    fn day10_sample() {
        let start = std::time::Instant::now();

        let input = r#"..F7.
.FJ|.
SJ.L7
|F--J
LJ..."#;
        let input = input.as_bytes();

        use std::str;
        println!("{:?}", str::from_utf8(&input));
        let furthest_tile = solve(input);

        assert_eq!(furthest_tile, 8usize);
        println!("Process in: {:?}", start.elapsed());
    }
}
