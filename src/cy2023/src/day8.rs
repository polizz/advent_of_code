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
}
