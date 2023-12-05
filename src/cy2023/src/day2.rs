#![allow(dead_code)]
use std::collections::BinaryHeap;

const RED_MAX: usize = 12;
const GREEN_MAX: usize = 13;
const BLUE_MAX: usize = 14;

#[derive(Debug)]
struct GameStatistic {
    id: usize,
    red: BinaryHeap<usize>,
    green: BinaryHeap<usize>,
    blue: BinaryHeap<usize>,
}

impl GameStatistic {
    fn new(id: usize) -> Self {
        GameStatistic {
            id,
            red: BinaryHeap::new(),
            green: BinaryHeap::new(),
            blue: BinaryHeap::new(),
        }
    }
}

type AggregatorFn =
    fn(red_max: usize, green_max: usize, blue_max: usize, acc: usize, id: usize) -> usize;

pub fn parse_file(file_content: String, aggregator_fn: AggregatorFn) -> usize {
    let game_stats = file_content.lines().map(|game_line| {
        let mut first_split = game_line.split(':');
        let mut gs = GameStatistic::new(
            first_split
                .next()
                .unwrap()
                .split(' ')
                .skip(1)
                .collect::<String>()
                .parse()
                .expect("Must have game ID first"),
        );

        first_split
            .next()
            .unwrap()
            .split(';')
            .for_each(|all_bags_for_game| {
                all_bags_for_game.split(',').for_each(|d| {
                    let num_cubes_and_color = d.trim().split(' ').collect::<Vec<&str>>();
                    let cube_num: usize = num_cubes_and_color[0]
                        .parse::<usize>()
                        .expect("Number of cubes should be a number");
                    let cube_color = num_cubes_and_color[1];

                    match cube_color {
                        "blue" => gs.blue.push(cube_num),
                        "red" => gs.red.push(cube_num),
                        "green" => gs.green.push(cube_num),
                        _ => panic!("not gonna happen"),
                    }
                });
            });

        gs
    });

    let games_aggregate = game_stats.fold(
        0,
        |acc,
         GameStatistic {
             id,
             mut red,
             mut green,
             mut blue,
         }| {
            let red_max = red.pop().expect("Must have a red");
            let green_max = green.pop().expect("Must have a green");
            let blue_max = blue.pop().expect("Must have a blue");

            aggregator_fn(red_max, green_max, blue_max, acc, id)
        },
    );

    games_aggregate
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::prelude::*;

    fn load_file() -> String {
        let mut file = File::open("src/fixtures/day2.txt").unwrap();
        let mut file_contents = String::new();
        let _ = file.read_to_string(&mut file_contents);

        file_contents
    }

    #[test]
    fn part2() {
        let start = std::time::Instant::now();

        fn aggregate(
            red_max: usize,
            green_max: usize,
            blue_max: usize,
            acc: usize,
            _id: usize,
        ) -> usize {
            acc + (red_max * green_max * blue_max)
        }

        let answer = parse_file(load_file(), aggregate);
        assert_eq!(answer, 71220usize);

        println!("Process in: {:?}", start.elapsed());
    }

    #[test]
    fn part1() {
        let start = std::time::Instant::now();

        fn aggregate(
            red_max: usize,
            green_max: usize,
            blue_max: usize,
            acc: usize,
            id: usize,
        ) -> usize {
            if red_max <= RED_MAX && green_max <= GREEN_MAX && blue_max <= BLUE_MAX {
                acc + id
            } else {
                acc
            }
        }

        let answer = parse_file(load_file(), aggregate);
        assert_eq!(answer, 2377usize);

        println!("Process in: {:?}", start.elapsed());
    }
}
