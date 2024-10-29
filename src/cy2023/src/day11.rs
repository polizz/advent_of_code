// --- Day 11: Cosmic Expansion ---
// You continue following signs for "Hot Springs" and eventually come across an observatory. The Elf within turns out to be a researcher studying cosmic expansion using the giant telescope here.
//
// He doesn't know anything about the missing machine parts; he's only visiting for this research project. However, he confirms that the hot springs are the next-closest area likely to have people; he'll even take you straight there once he's done with today's observation analysis.
//
// Maybe you can help him with the analysis to speed things up?
//
// The researcher has collected a bunch of data and compiled the data into a single giant image (your puzzle input). The image includes empty space (.) and galaxies (#). For example:
//
// ...#......
// .......#..
// #.........
// ..........
// ......#...
// .#........
// .........#
// ..........
// .......#..
// #...#.....
// The researcher is trying to figure out the sum of the lengths of the shortest path between every pair of galaxies. However, there's a catch: the universe expanded in the time it took the light from those galaxies to reach the observatory.
//
// Due to something involving gravitational effects, only some space expands. In fact, the result is that any rows or columns that contain no galaxies should all actually be twice as big.
//
// In the above example, three columns and two rows contain no galaxies:
//
//    v  v  v
//  ...#......
//  .......#..
//  #.........
// >..........<
//  ......#...
//  .#........
//  .........#
// >..........<
//  .......#..
//  #...#.....
//    ^  ^  ^
// These rows and columns need to be twice as big; the result of cosmic expansion therefore looks like this:
//
// ....#........
// .........#...
// #............
// .............
// .............
// ........#....
// .#...........
// ............#
// .............
// .............
// .........#...
// #....#.......
// Equipped with this expanded universe, the shortest path between every pair of galaxies can be found. It can help to assign every galaxy a unique number:
//
// ....1........
// .........2...
// 3............
// .............
// .............
// ........4....
// .5...........
// ............6
// .............
// .............
// .........7...
// 8....9.......
// In these 9 galaxies, there are 36 pairs. Only count each pair once; order within the pair doesn't matter. For each pair, find any shortest path between the two galaxies using only steps that move up, down, left, or right exactly one . or # at a time. (The shortest path between two galaxies is allowed to pass through another galaxy.)
//
// For example, here is one of the shortest paths between galaxies 5 and 9:
//
// ....1........
// .........2...
// 3............
// .............
// .............
// ........4....
// .5...........
// .##.........6
// ..##.........
// ...##........
// ....##...7...
// 8....9.......
// This path has length 9 because it takes a minimum of nine steps to get from galaxy 5 to galaxy 9 (the eight locations marked # plus the step onto galaxy 9 itself). Here are some other example shortest path lengths:
//
// Between galaxy 1 and galaxy 7: 15
// Between galaxy 3 and galaxy 6: 17
// Between galaxy 8 and galaxy 9: 5
// In this example, after expanding the universe, the sum of the shortest path between all 36 pairs of galaxies is 374.
//
// Expand the universe, then find the length of the shortest path between every pair of galaxies. What is the sum of these lengths?

// --- Part Two ---
// The galaxies are much older (and thus much farther apart) than the researcher initially estimated.
//
// Now, instead of the expansion you did before, make each empty row or column one million times larger. That is, each empty row should be replaced with 1000000 empty rows, and each empty column should be replaced with 1000000 empty columns.
//
// (In the example above, if each empty row or column were merely 10 times larger, the sum of the shortest paths between every pair of galaxies would be 1030. If each empty row or column were merely 100 times larger, the sum of the shortest paths between every pair of galaxies would be 8410. However, your universe will need to expand far beyond these values.)
//
// Starting with the same initial image, expand the universe according to these new rules, then find the length of the shortest path between every pair of galaxies. What is the sum of these lengths?

use std::collections::HashSet;

type Coordinate = (usize, usize);
type GalacticCoordinates = Vec<Coordinate>;

#[derive(Debug, Clone)]
struct WalkInfo {
    src_point: Coordinate,
    destinations: Vec<DestInfo>,
}

#[derive(Debug, Clone)]
struct SlopeInfo {
    x: usize,
    x_sign: bool,
    y: usize,
    y_sign: bool,
}

#[derive(Debug, Clone)]
struct DestInfo {
    dst_point: Coordinate,
    slope_pair: SlopeInfo,
}

type FnDistanceCalc = fn(GalacticCoordinates) -> usize;

fn solve(map: &[u8], expansion_factor: usize, calc_dist_fn: FnDistanceCalc) -> usize {
    let expanded_map = expand_map(map, expansion_factor);
    calc_dist_fn(expanded_map)
}

fn get_slope_pairs(expanded_map: GalacticCoordinates) -> Vec<WalkInfo> {
    let coord_len = expanded_map.len();
    let mut coord_window_start = 0;

    let mut walking_info = vec![];

    while coord_window_start < coord_len {
        let mut dest_infos = vec![];
        let src_x = expanded_map[coord_window_start].0;
        let src_y = expanded_map[coord_window_start].1;

        for peer in coord_window_start + 1..expanded_map.len() {
            let dst_x = expanded_map[peer].0;
            let dst_y = expanded_map[peer].1;

            let mut slope_x = dst_x as isize - src_x as isize;
            let mut slope_y = dst_y as isize - src_y as isize;

            let x_sign = slope_x < 0;
            let y_sign = slope_y < 0;

            if slope_x >= slope_y && slope_y > 0 {
                slope_x = slope_x / slope_y;
                slope_y = 1;
            } else if slope_y >= slope_x && slope_x > 0 {
                slope_y = slope_y / slope_x;
                slope_x = 1;
            } else if x_sign && !y_sign {
                if slope_x.abs() > slope_y {
                    slope_x = slope_x.abs() / slope_y;
                    slope_y = 1;
                } else {
                    slope_y = slope_y / slope_x.abs();
                    slope_x = 1;
                }
            } else if !x_sign && y_sign {
                if slope_y.abs() > slope_x {
                    slope_y = slope_y.abs() / slope_x;
                    slope_x = 1;
                } else {
                    slope_x = slope_x / slope_y.abs();
                    slope_y = 1;
                }
            }

            dest_infos.push(DestInfo {
                dst_point: (dst_x, dst_y),
                slope_pair: SlopeInfo {
                    x: slope_x as usize,
                    x_sign,
                    y: slope_y as usize,
                    y_sign,
                },
            });
        }

        walking_info.push(WalkInfo {
            src_point: (src_x, src_y),
            destinations: dest_infos,
        });
        coord_window_start += 1;
    }

    walking_info
}

fn calculate_approx_distances(expanded_map: GalacticCoordinates) -> usize {
    fn walk_to_point(src: Coordinate, dst_info: &DestInfo) -> usize {
        let SlopeInfo {
            x_sign,
            y_sign,
            x: step_x,
            y: step_y,
        } = dst_info.slope_pair;
        let mut nav_point = src;
        let (dst_x, dst_y) = dst_info.dst_point;
        let mut total_steps = 0usize;

        loop {
            if nav_point.0 != dst_x {
                for x in 0..step_x {
                    total_steps += 1;

                    if x_sign {
                        nav_point.0 -= 1;
                        // up
                        if nav_point.0 <= dst_x {
                            break;
                        }
                    } else {
                        nav_point.0 += 1;
                        // down
                        if nav_point.0 >= dst_x {
                            break;
                        }
                    }
                }
            }

            if nav_point.1 != dst_y {
                for y in 0..step_y {
                    total_steps += 1;

                    if y_sign {
                        nav_point.1 -= 1;
                        // left
                        if nav_point.1 <= dst_y {
                            break;
                        }
                    } else {
                        nav_point.1 += 1;
                        // right
                        if nav_point.1 >= dst_y {
                            break;
                        }
                    }
                }
            }

            if x_sign && y_sign {
                // up and to left
                if nav_point.0 <= dst_x && nav_point.1 <= dst_y {
                    break;
                }
            } else if x_sign && !y_sign {
                // up or up and right
                if nav_point.0 <= dst_x && nav_point.1 >= dst_y {
                    break;
                }
            } else if !x_sign && !y_sign {
                // down or down and right
                if nav_point.0 >= dst_x && nav_point.1 >= dst_y {
                    break;
                }
            } else if !x_sign && y_sign {
                // down or down and left
                if nav_point.0 >= dst_x && nav_point.1 <= dst_y {
                    break;
                }
            }
        }

        // println!(
        //     "Tot Steps for ({:?}, {:?}) to ({:?}, {:?}) = {:?} taking steps ({:?}, {:?})",
        //     src.0, src.1, dst_x, dst_y, total_steps, step_x, step_y
        // );
        total_steps
    }

    let slopes = get_slope_pairs(expanded_map);
    let total_dests = slopes.iter().fold(0, |acc, s| acc + s.destinations.len());
    println!(
        "Begin walking {:?} slope entries with {:?} total slope destinations.",
        slopes.len(),
        total_dests
    );

    let walking_sum: usize = slopes.iter().fold(0, |sum, walk_info| {
        let WalkInfo {
            src_point: (src_x, src_y),
            destinations: dest,
        } = walk_info;

        let distance_sum = dest.iter().fold(0, |sum, d| {
            let s = sum + walk_to_point((*src_x, *src_y), d);
            println!("Completed walk from ({:?}, {:?}) to {:?}", src_x, src_y, d);
            s
        });

        distance_sum + sum
    });

    walking_sum
}

type Galaxy = (usize, usize);

fn expand_map(map: &[u8], expansion_factor: usize) -> GalacticCoordinates {
    let n = map
        .iter()
        .position(|&c| c == b'\n')
        .expect("format defines newline terminator");
    let num_newlines = map.iter().filter(|c| **c == b'\n').count();
    let m = (map.len() - num_newlines) / n;

    let mut used_rows = HashSet::new();
    let mut used_cols = HashSet::new();

    let mut new_lines = 0usize;
    let mut galaxies: Vec<Galaxy> = map.iter().enumerate().fold(vec![], |mut acc, (ix, ch)| {
        let ix_mod = ix - new_lines;
        let (row, col) = (ix_mod / m, ix_mod % m);
        match *ch {
            b'#' => {
                used_rows.insert(row);
                used_cols.insert(col);

                acc.push((row, col));
            }
            b'\n' => new_lines += 1,
            _ => {}
        }
        acc
    });

    let vacant_rows: Vec<usize> = (0..m).filter(|col| !used_rows.contains(&col)).collect();
    let vacant_cols: Vec<usize> = (0..n).filter(|col| !used_cols.contains(&col)).collect();

    println!(
        "Galaxies: {:?}, Vacant rows: {:?}, Vacant cols: {:?}",
        galaxies.len(),
        vacant_rows.len(),
        vacant_cols.len()
    );

    for galaxy in 0..galaxies.len() {
        let mut gal_x_exp = 0;
        let mut gal_y_exp = 0;

        vacant_rows.iter().for_each(|expansion_row| {
            if galaxies[galaxy].0 >= *expansion_row {
                gal_x_exp += expansion_factor
            }
        });

        vacant_cols.iter().for_each(|expansion_col| {
            if galaxies[galaxy].1 >= *expansion_col {
                gal_y_exp += expansion_factor
            }
        });

        galaxies[galaxy].1 += gal_y_exp;
        galaxies[galaxy].0 += gal_x_exp;
    }

    galaxies
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day11_part2_file() {
        let start = std::time::Instant::now();
        let input = include_bytes!("fixtures/day11.txt");

        let total_min_spans = solve(input, 999999, calculate_approx_distances);

        assert_eq!(total_min_spans, 597714117556usize);
        println!("Process in: {:?}", start.elapsed());
    }

    #[test]
    fn day11_part2_10small() {
        let start = std::time::Instant::now();

        let input = r#"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#....."#;

        let input = input.as_bytes();
        let total_min_spans = solve(input, 9, calculate_approx_distances);

        assert_eq!(total_min_spans, 1030usize);
        println!("Process in: {:?}", start.elapsed());
    }

    #[test]
    fn day11_part2_100small() {
        let start = std::time::Instant::now();

        let input = r#"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#....."#;

        let input = input.as_bytes();
        let total_min_spans = solve(input, 99, calculate_approx_distances);

        assert_eq!(total_min_spans, 8410usize);
        println!("Process in: {:?}", start.elapsed());
    }

    #[test]
    fn day11_file1() {
        let start = std::time::Instant::now();
        let input = include_bytes!("fixtures/day11.txt");

        let total_min_spans = solve(input, 1, calculate_approx_distances);

        assert_eq!(total_min_spans, 9312968usize);
        println!("Process in: {:?}", start.elapsed());
    }

    #[test]
    fn day11_sample1() {
        let start = std::time::Instant::now();
        let input = r#"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#....."#;

        let input = input.as_bytes();

        let total_min_spans = solve(input, 1, calculate_approx_distances);

        assert_eq!(total_min_spans, 374usize);
        println!("Process in: {:?}", start.elapsed());
    }
}

// This only works for small matrices
// fn calculate_discrete_distances(expanded_map: GalacticCoordinates) -> usize {
//     let slopes = get_slope_pairs(expanded_map);
//     let sum = slopes.iter().fold(0, |sum, (x, y)| sum + x.abs() + y.abs());
//     sum as usize
//     /* point slope: y-y1 = m(x-x1)
//                  m = (y2-y1) / (x2-x1)
//
//     ## Algorithm for calculating shortest paths:
//
//     After expansion
//      ....#........
//      .........a...  <
//      #............
//      .............
//      .............
//      ........b....
//      .e...........
//      ............c  <
//      .............
//      .............
//      .........f...
//      #....d.......
//           ^
//      Coordinate spatial processing of points:
//       a -> d
//      (1, 9) -> (5,11)
//      m = 2/4
//      d = sqrt(4^2+2^2) = sqrt(20) = 4.47
//
//       d -> e
//      (5,11) -> (1, 6)
//      m = 5/4
//      d = 6.4
//
//       d -> b
//      (5, 11) -> (8, 5)
//      m = 6/3
//      d = 6.7
//
//       d -> c
//      (5, 11) -> (12, 7)
//      m = 11-7 / 5 - 12, -4/7
//      d = 8.06
//
//      * for slope m = Y/X, when d > Y+X, min(path) == Y+X
//       d -> f
//      (5, 11) -> (9, 10)
//      m = -1/4
//      d = 4.12
//
//      Special cases for infinite slope (vertical) and 0 slope (horizontal)
//      when m = 0, min(path) == abs(y1 - y2)
//      when m = inf, min(path) == abs(x1 - x2)
//
//     */
// }
