// --- Day 8: Haunted Wasteland ---
// You're still riding a camel across Desert Island when you spot a sandstorm quickly approaching. When you turn to warn the Elf, she disappears before your eyes! To be fair, she had just finished warning you about ghosts a few minutes ago.
//
// One of the camel's pouches is labeled "maps" - sure enough, it's full of documents (your puzzle input) about how to navigate the desert. At least, you're pretty sure that's what they are; one of the documents contains a list of left/right instructions, and the rest of the documents seem to describe some kind of network of labeled nodes.
//
// It seems like you're meant to use the left/right instructions to navigate the network. Perhaps if you have the camel follow the same instructions, you can escape the haunted wasteland!
//
// After examining the maps for a bit, two nodes stick out: AAA and ZZZ. You feel like AAA is where you are now, and you have to follow the left/right instructions until you reach ZZZ.
//
// This format defines each node of the network individually. For example:
//
// RL
//
// AAA = (BBB, CCC)
// BBB = (DDD, EEE)
// CCC = (ZZZ, GGG)
// DDD = (DDD, DDD)
// EEE = (EEE, EEE)
// GGG = (GGG, GGG)
// ZZZ = (ZZZ, ZZZ)
// Starting with AAA, you need to look up the next element based on the next left/right instruction in your input. In this example, start with AAA and go right (R) by choosing the right element of AAA, CCC. Then, L means to choose the left element of CCC, ZZZ. By following the left/right instructions, you reach ZZZ in 2 steps.
//
// Of course, you might not find ZZZ right away. If you run out of left/right instructions, repeat the whole sequence of instructions as necessary: RL really means RLRLRLRLRLRLRLRL... and so on. For example, here is a situation that takes 6 steps to reach ZZZ:
//
// LLR
//
// AAA = (BBB, BBB)
// BBB = (AAA, ZZZ)
// ZZZ = (ZZZ, ZZZ)
// Starting at AAA, follow the left/right instructions. How many steps are required to reach ZZZ?

// --- Part Two ---
// The sandstorm is upon you and you aren't any closer to escaping the wasteland. You had the camel follow the instructions, but you've barely left your starting position. It's going to take significantly more steps to escape!
//
// What if the map isn't for people - what if the map is for ghosts? Are ghosts even bound by the laws of spacetime? Only one way to find out.
//
// After examining the maps a bit longer, your attention is drawn to a curious fact: the number of nodes with names ending in A is equal to the number ending in Z! If you were a ghost, you'd probably just start at every node that ends with A and follow all of the paths at the same time until they all simultaneously end up at nodes that end with Z.
//
// For example:
//
// LR
//
// 11A = (11B, XXX)
// 11B = (XXX, 11Z)
// 11Z = (11B, XXX)
// 22A = (22B, XXX)
// 22B = (22C, 22C)
// 22C = (22Z, 22Z)
// 22Z = (22B, 22B)
// XXX = (XXX, XXX)
// Here, there are two starting nodes, 11A and 22A (because they both end with A). As you follow each left/right instruction, use that instruction to simultaneously navigate away from both nodes you're currently on. Repeat this process until all of the nodes you're currently on end with Z. (If only some of the nodes you're on end with Z, they act like any other node and you continue as normal.) In this example, you would proceed as follows:
//
// Step 0: You are at 11A and 22A.
// Step 1: You choose all of the left paths, leading you to 11B and 22B.
// Step 2: You choose all of the right paths, leading you to 11Z and 22C.
// Step 3: You choose all of the left paths, leading you to 11B and 22Z.
// Step 4: You choose all of the right paths, leading you to 11Z and 22B.
// Step 5: You choose all of the left paths, leading you to 11B and 22C.
// Step 6: You choose all of the right paths, leading you to 11Z and 22Z.
// So, in this example, you end up entirely on nodes that end in Z after 6 steps.
//
// Simultaneously start on every node that ends with A. How many steps does it take before you're only on nodes that end with Z?

use core::panic;
use std::collections::HashMap;

pub fn navigate_graph_alternate_bruteforce(
    directions: &Vec<usize>,
    nodes: &HashMap<&str, Nodes>,
    starts: Vec<&str>,
) -> usize {
    let mut steps = 0;

    let mut next_nodes = starts;
    let mut intermediate: Vec<&str> = vec![];
    'outer: loop {
        for nav_step in directions {
            for node in next_nodes.iter() {
                let child_nodes = nodes.get(node).expect("Must exist");
                let next_dir_key = match nav_step {
                    0 => child_nodes.0,
                    1 => child_nodes.1,
                    _ => panic!("Won't happen"),
                };

                intermediate.push(next_dir_key);
            }

            steps += 1;

            if intermediate.iter().all(|&n| n.ends_with('Z')) {
                println!("have all z's: {:?}", &intermediate);
                break 'outer;
            }

            next_nodes = intermediate.clone();
            intermediate.clear();
        }
    }

    steps
}

pub fn navigate_graph(
    directions: &Vec<usize>,
    nodes: &HashMap<&str, Nodes>,
    targets: NavTargets,
) -> usize {
    let mut steps = 0;

    let mut next_nodes = nodes.get(targets.0).expect("Must have it");

    println!("directions: {:?}", &directions);
    let mut next_dir_key = "";

    'outer: loop {
        for nav_step in directions {
            next_dir_key = match nav_step {
                0 => next_nodes.0,
                1 => next_nodes.1,
                _ => panic!("Won't happen"),
            };

            steps += 1;
            // println!("next_dir_key: {:?}", &next_dir_key);
            next_nodes = nodes.get(next_dir_key).expect("Must exist");

            // if next_dir_key == targets.1 {
            //     break 'outer;
            // }
        }
        if next_dir_key == targets.1 {
            break 'outer;
        }
        if steps % 100_000 == 0 {
            println!("Steps: {:?}", &steps);
        }
    }

    steps
}

pub fn navigate_graph_alternate(
    directions: &Vec<usize>,
    nodes: &HashMap<&str, Nodes>,
    starts: Vec<&str>,
) -> usize {
    println!("Starts: {:?}", &starts);
    let z_found_at: Vec<usize> = starts
        .iter()
        .map(|&n| {
            let mut steps = 0usize;
            println!("Node: {:?}", n);
            let mut link_node = n;

            'outer: loop {
                for nav_step in directions {
                    let curr_node = nodes.get(link_node).expect("Must exist");

                    if link_node.ends_with('Z') {
                        break 'outer;
                    }

                    link_node = match nav_step {
                        0 => curr_node.0,
                        1 => curr_node.1,
                        _ => panic!("Won't happen"),
                    };

                    steps += 1;
                }
            }

            steps
        })
        .collect();

    println!("Z found at for all: {:?}", &z_found_at);

    0
}

pub fn parse_graph_def<'t>(input: &'t str) -> (Vec<usize>, HashMap<&str, Nodes>, NavTargets) {
    let mut lines = input.lines();
    let dirs: Vec<usize> = lines
        .next()
        .unwrap()
        .chars()
        .map(|c| match c {
            'L' => 0,
            'R' => 1,
            _ => panic!("wth"),
        })
        .collect();

    let mut ht: HashMap<&str, Nodes> = HashMap::new();
    let mut nav_targets: Vec<&str> = vec![];
    let mut lines: Vec<&str> = lines
        .skip(1) // blank line
        .collect();
    lines.sort();
    lines.iter().for_each(|&line| {
        let mut splitter = line.split('=');
        let node_name = splitter.next().unwrap().trim();
        let node_dests = splitter.next().unwrap().trim();

        let mut node_dest_splitter = node_dests.split(',');
        let left = node_dest_splitter.next().unwrap().trim_matches('(').trim();
        let right = node_dest_splitter.next().unwrap().trim_matches(')').trim();

        // dbg!((left, right));

        nav_targets.push(node_name);
        let _ = ht.insert(node_name, (left, right));
    });

    let start = nav_targets[0];
    let end = nav_targets[nav_targets.len() - 1];

    (dirs, ht, (start, end))
}

pub fn parse_graph_def_alternate<'t>(
    input: &'t str,
) -> (Vec<usize>, HashMap<&str, Nodes>, Vec<&str>) {
    let mut lines = input.lines();
    let dirs: Vec<usize> = lines
        .next()
        .unwrap()
        .chars()
        .map(|c| match c {
            'L' => 0,
            'R' => 1,
            _ => panic!("wth"),
        })
        .collect();

    let mut ht: HashMap<&str, Nodes> = HashMap::new();
    let mut starts: Vec<&str> = vec![];

    let lines: Vec<&str> = lines
        .skip(1) // blank line
        .collect();
    // lines.sort();
    lines.iter().for_each(|&line| {
        let mut splitter = line.split('=');
        let node_name = splitter.next().unwrap().trim();
        let node_dests = splitter.next().unwrap().trim();

        let mut node_dest_splitter = node_dests.split(',');
        let left = node_dest_splitter.next().unwrap().trim_matches('(').trim();
        let right = node_dest_splitter.next().unwrap().trim_matches(')').trim();

        if node_name.ends_with('A') {
            starts.push(node_name);
        }

        let _ = ht.insert(node_name, (left, right));
    });

    (dirs, ht, starts)
}

pub type NavTargets<'t> = (&'t str, &'t str);
pub type Nodes<'t> = (&'t str, &'t str);
// pub type Nodes = (usize, usize);

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::prelude::*;

    fn load_file() -> String {
        let mut file = File::open("src/fixtures/day8.txt").unwrap();
        let mut file_contents = String::new();
        let _ = file.read_to_string(&mut file_contents);
        file_contents
    }

    #[test]
    fn day8_file3() {
        let start = std::time::Instant::now();

        let input = load_file();
        let (directions, nodes, targets) = parse_graph_def_alternate(&input);
        println!(
            "dir: {:?}, nodes: {:?}, targets: {:?}",
            &directions, &nodes, &targets
        );
        let step_count = navigate_graph_alternate_bruteforce(&directions, &nodes, targets);

        assert_eq!(step_count, 19631usize);
        println!("Process in: {:?}", start.elapsed());
    }

    #[test]
    fn day8_test3() {
        let start = std::time::Instant::now();

        let input = r#"LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)"#;

        let (directions, nodes, targets) = parse_graph_def_alternate(&input);
        println!(
            "dir: {:?}, nodes: {:?}, targets: {:?}",
            &directions, &nodes, &targets
        );
        let step_count = navigate_graph_alternate(&directions, &nodes, targets);
        assert_eq!(step_count, 6usize);

        println!("Process in: {:?}", start.elapsed());
    }

    #[test]
    fn day8_file() {
        let start = std::time::Instant::now();

        let input = load_file();
        let (directions, nodes, targets) = parse_graph_def(&input);
        let step_count = navigate_graph(&directions, &nodes, targets);

        assert_eq!(step_count, 19631usize);
        println!("Process in: {:?}", start.elapsed());
    }

    #[test]
    fn day8_test2() {
        let start = std::time::Instant::now();

        let input = r#"LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)"#;

        let (directions, nodes, targets) = parse_graph_def(input);
        let step_count = navigate_graph(&directions, &nodes, targets);

        assert_eq!(step_count, 6usize);

        println!("Process in: {:?}", start.elapsed());
    }

    #[test]
    fn day8_test() {
        let start = std::time::Instant::now();

        let input = r#"RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)"#;

        let (directions, nodes, targets) = parse_graph_def(input);
        let step_count = navigate_graph(&directions, &nodes, targets);

        assert_eq!(step_count, 2usize);

        println!("Process in: {:?}", start.elapsed());
    }

    #[test]
    fn day8_other2() {
        let input = include_bytes!("./fixtures/day8.txt");
        let split = input.iter().position(|&c| c == b'\n').unwrap();

        let mut map = [0u32; 0b11001_11001_11001 + 1];
        let enc = |n: &[u8]| {
            ((n[0] - b'A') as u32) << 10 | ((n[1] - b'A') as u32) << 5 | (n[2] - b'A') as u32
        };
        input[split + 2..].split(|&c| c == b'\n').for_each(|node| {
            map[enc(&node[0..3]) as usize] = enc(&node[7..10]) | enc(&node[12..15]) << 16;
        });

        println!(
            "{}",
            input[0..split]
                .iter()
                .cycle()
                .scan(enc(b"AAA"), |node, step| {
                    *node = if step == &b'L' {
                        map[*node as usize] & u16::MAX as u32
                    } else {
                        map[*node as usize] >> 16
                    };
                    Some(*node & 0b11111 == (b'Z' - b'A') as u32)
                })
                .position(|node| node)
                .unwrap()
                + 1
        );
    }

    #[test]
    fn day8_other1() {
        // use std::str;
        let input = include_bytes!("./fixtures/day8.txt");
        let split = input.iter().position(|&c| c == b'\n').unwrap();

        let mut map = [0u32; 0b11001_11001_11001 + 1];
        let enc = |n: &[u8]| {
            ((n[0] - b'A') as u32) << 10 | ((n[1] - b'A') as u32) << 5 | (n[2] - b'A') as u32
        };
        input[split + 2..].split(|&c| c == b'\n').for_each(|node| {
            // println!("node: {:?}", str::from_utf8(node));
            if node.len() > 0usize {
                map[enc(&node[0..3]) as usize] = enc(&node[7..10]) | enc(&node[12..15]) << 16;
            }
        });

        let step_count = input[0..split]
            .iter()
            .cycle()
            .scan(enc(b"AAA"), |node, step| {
                *node = if step == &b'L' {
                    map[*node as usize] & u16::MAX as u32
                } else {
                    map[*node as usize] >> 16
                };
                Some(*node & 0b11111 == (b'Z' - b'A') as u32)
            })
            .position(|node| node)
            .unwrap()
            + 1;
        assert_eq!(step_count, 19631usize);
    }
}
