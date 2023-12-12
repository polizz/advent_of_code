// sum of all part numbers that are touching a non-period symbol
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_assignments)]
use std::collections::{HashSet, VecDeque};
use std::sync::OnceLock;

type Location = (usize, usize);
type LocationDeltas = (isize, isize);

fn init_deltas() -> &'static [LocationDeltas; 8] {
    static SCAN_POS_DELTAS: OnceLock<[LocationDeltas; 8]> = OnceLock::new();
    SCAN_POS_DELTAS.get_or_init(|| {
        [
            (-1, 0),  //Up
            (-1, 1),  //UpDiagRight
            (0, 1),   //Right
            (1, 1),   //DownDiagRight
            (1, 0),   //Down
            (1, -1),  //DownDiagLeft
            (0, -1),  //Left
            (-1, -1), //UpDiagLeft
        ]
    })
}

fn get_scan_destinations(
    curr_position: Location,
    m: usize,
    n: usize,
) -> impl Iterator<Item = Location> {
    let ld = init_deltas();
    let (curr_row, curr_col) = curr_position;

    ld.iter().filter_map(move |(r, c)| {
        let new_row = curr_row as isize + r;
        let new_col = curr_col as isize + c;

        if new_row >= 0 && new_row < m as isize && new_col >= 0 && new_col < n as isize {
            Some((new_row as usize, new_col as usize))
        } else {
            None
        }
    })
}

fn is_part_number(curr_position: Location, grid: &Vec<Vec<char>>) -> bool {
    let m = grid.len();
    let n = grid[0].len();

    let tf = get_scan_destinations(curr_position, m, n)
        // .inspect(move |x| println!("Destinations for {} are: {}", &curr_position, &x))
        .any(|(mnext, nnext)| match grid[mnext][nnext] {
            '*' | '@' | '#' | '$' | '%' | '&' | '/' | '=' | '+' | '-' => true,
            _ => false,
        });

    tf
}

fn read_number(
    loc: Location,
    grid: &Vec<Vec<char>>,
    seen_digit_locs: &mut HashSet<Location>,
    adjacent_gears: &mut VecDeque<Location>,
    seen_gear_locs: &mut HashSet<Location>,
    M: usize,
    N: usize,
) -> Option<usize> {
    // read far left on number and then read entire number to right
    let mut number_scanned = String::new();
    let (m, n) = loc;

    for n_bak in (0..n).rev() {
        if !(grid[m][n_bak].is_digit(10)) {
            for n_fwd in n_bak + 1..grid[0].len() {
                if grid[m][n_fwd].is_digit(10) {
                    seen_digit_locs.insert((m, n_fwd));
                    number_scanned.push(grid[m][n_fwd]);

                    let dests =
                        get_scan_destinations((m, n_fwd), M, N).filter_map(|loc| {
                            match grid[loc.0][loc.1] {
                                '*' if !seen_gear_locs.contains(&loc) => {
                                    seen_digit_locs.insert((m, n));
                                    Some(loc)
                                }
                                _ => None,
                            }
                        });
                    adjacent_gears.extend(dests);
                } else {
                    if number_scanned.len() > 0 {
                        let num_r = number_scanned.parse::<usize>().unwrap();
                        return Some(num_r);
                    }

                    return None;
                }
            }
        } else if n_bak == 0 {
            for n_fwd in n_bak..grid[0].len() {
                if grid[m][n_fwd].is_digit(10) {
                    seen_digit_locs.insert((m, n_fwd));
                    number_scanned.push(grid[m][n_fwd]);

                    let dests =
                        get_scan_destinations((m, n_fwd), M, N).filter_map(|loc| {
                            match grid[loc.0][loc.1] {
                                '*' if !seen_gear_locs.contains(&loc) => {
                                    seen_digit_locs.insert((m, n));
                                    Some(loc)
                                }
                                _ => None,
                            }
                        });
                    adjacent_gears.extend(dests);
                } else {
                    if number_scanned.len() > 0 {
                        let num_r = number_scanned.parse::<usize>().unwrap();
                        return Some(num_r);
                    }

                    return None;
                }
            }
        }
    }

    None
}

pub fn parse_gears(grid: &Vec<Vec<char>>) -> usize {
    let M = grid.len();
    let N = grid[0].len();

    let mut solution: usize = 0;
    let mut running_numbers_list: Vec<usize> = vec![];
    let mut seen_digit_locs: HashSet<Location> = HashSet::new();
    let mut seen_gear_locs: HashSet<Location> = HashSet::new();
    let mut adjacent_gears: VecDeque<Location> = VecDeque::new();
    let mut adjacent_nums: VecDeque<Location> = VecDeque::new();

    for (m, row) in grid.iter().enumerate() {
        for (n, b) in row.iter().enumerate() {
            if seen_gear_locs.contains(&(m, n)) || seen_digit_locs.contains(&(m, n)) {
                continue;
            }

            match b {
                '*' => {
                    adjacent_gears.push_back((m, n));

                    while let Some(gear) = adjacent_gears.pop_front() {
                        seen_gear_locs.insert(gear);

                        // get candidate locations for surrounding
                        // numbers and push on to queue
                        let dests = get_scan_destinations(gear, M, N).filter_map(|loc| match grid
                            [loc.0][loc.1]
                        {
                            '0'..='9' if !seen_digit_locs.contains(&loc) => Some(loc),
                            _ => None,
                        });

                        adjacent_nums.extend(dests);

                        while let Some(cand_num) = adjacent_nums.pop_front() {
                            if !seen_digit_locs.contains(&cand_num) {
                                if let Some(num) = read_number(
                                    cand_num,
                                    grid,
                                    &mut seen_digit_locs,
                                    &mut adjacent_gears,
                                    &mut seen_gear_locs,
                                    M,
                                    N,
                                ) {
                                    running_numbers_list.push(num);
                                }
                            }
                        }
                    }
                    if running_numbers_list.len() > 1 {
                        // we have more than one number surrounding this gear
                        // multiply them all and add to running power value
                        solution =
                            solution + running_numbers_list.iter().fold(1, |acc, num| num * acc);
                    }
                    running_numbers_list.clear();
                }
                _ => continue,
            }
        }
    }

    solution
}

pub fn parse_file(grid: &Vec<Vec<char>>) -> usize {
    let mut running_number = String::new();
    let mut valid_numbers: Vec<usize> = vec![];

    let mut part_found = false;
    for (m, row) in grid.iter().enumerate() {
        for (n, b) in row.iter().enumerate() {
            // println!("Loc: {:?}, candidate : {}", &(m, n), &b);
            match b {
                '0'..='9' => {
                    running_number.push(b.clone());

                    if !part_found {
                        // println!("Checking is part...");
                        if is_part_number((m, n), &grid) {
                            part_found = true;
                        }
                    }
                }
                _ => {
                    // println!("Other match: {}", b);
                    if part_found && running_number.len() > 0 {
                        // println!("Saving number: {}", running_number);
                        valid_numbers
                            .push(running_number.parse().expect("Should be valid a number"));
                    }
                    running_number.clear();
                    part_found = false;
                }
            }
        }

        if part_found && running_number.len() > 0 {
            // println!("Saving number: {}", running_number);
            valid_numbers.push(running_number.parse().expect("Should be valid a number"));
        }
        running_number.clear();
        part_found = false;
    }

    if part_found && running_number.len() > 0 {
        // println!("Saving number: {}", running_number);
        valid_numbers.push(running_number.parse().expect("Should be valid a number"));
    }

    println!("valid_numbers: {:?}", &valid_numbers);

    valid_numbers.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::prelude::*;

    fn load_file() -> String {
        let mut file = File::open("src/fixtures/day3.txt").unwrap();
        let mut file_contents = String::new();
        let _ = file.read_to_string(&mut file_contents);

        init_deltas();

        file_contents
    }

    #[test]
    fn day3_gear() {
        let start = std::time::Instant::now();
        let input = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#;

        let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
        init_deltas();

        let answer = parse_gears(&grid);
        assert_eq!(answer, 467835usize);

        println!("Process in: {:?}", start.elapsed());
    }

    #[test]
    fn day3_part2() {
        let start = std::time::Instant::now();

        let input = load_file();
        let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
        let answer = parse_gears(&grid);

        assert_eq!(answer, 78915902usize);

        println!("Process in: {:?}", start.elapsed());
    }

    #[test]
    fn day3_part1() {
        let start = std::time::Instant::now();

        let input = load_file();
        let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
        let answer = parse_file(&grid);

        assert_eq!(answer, 514969usize);

        println!("Process in: {:?}", start.elapsed());
    }

    #[test]
    fn day3_star() {
        let start = std::time::Instant::now();

        let input = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664*598..
"#;

        let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
        init_deltas();

        let answer = parse_file(&grid);
        assert_eq!(answer, 4361usize);

        println!("Process in: {:?}", start.elapsed());
    }

    #[test]
    fn day3_issue1() {
        let start = std::time::Instant::now();

        let input = r#"467..114..
...*......
..35...335
821...#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
"#;

        let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
        init_deltas();

        let answer = parse_file(&grid);
        assert_eq!(answer, 4361usize);

        println!("Process in: {:?}", start.elapsed());
    }

    #[test]
    fn day3_test() {
        let start = std::time::Instant::now();

        let input = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
"#;

        let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
        init_deltas();

        let answer = parse_file(&grid);
        assert_eq!(answer, 4361usize);

        println!("Process in: {:?}", start.elapsed());
    }
}
